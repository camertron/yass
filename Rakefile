# frozen_string_literal: true

require "bundler/gem_tasks"
require "rspec/core/rake_task"

RSpec::Core::RakeTask.new(:spec)

require "rb_sys/extensiontask"

GEMSPEC = Gem::Specification.load("yass.gemspec")

RbSys::ExtensionTask.new("yass", GEMSPEC) do |ext|
  ext.lib_dir = "lib/yass"
  ext.cross_compile = true
  ext.cross_platform = [
    "x86_64-linux",
    "aarch64-linux",
    "x86_64-darwin",
    "arm64-darwin",
    "x64-mingw-ucrt",
  ]
end

task default: %i[compile spec]

namespace :spec do
  desc 'Run full specs suit'
  task full: [:full_spec_env, :compile, :spec]

  task :full_spec_env do
    ENV['FULL_SPEC'] = 'true'
  end
end

# For whatever reason, the :native task that comes with rb-sys doesn't work, so
# we define our own
task :native, [:platform] do |task, args|
  env = if args[:platform] =~ /mingw/
    # custom docker image that includes Python, which stylo seems to need
    sh(
      "docker", "build",
      "--platform", "linux/amd64",
      "-f", "dockerfiles/windows.dockerfile",
      "-t", "camertron/rbsys-x64-mingw-ucrt:latest",
      "dockerfiles/"
    )

    { "RCD_IMAGE" => "camertron/rbsys-x64-mingw-ucrt:latest" }
  else
    {}
  end

  sh(env, "bundle", "exec", "rb-sys-dock", "--platform", args[:platform], "--build")
end

task :codegen do
  require "yass"
  require_relative "codegen/rust_file_set"

  general_files = Yass::Codegen::RustFileSet::new(
    Dir.glob("ext/yass/src/general/**/*.rs")
  )

  File.write("lib/yass/general.rb", <<~RUBY)
    # frozen_string_literal: true

    #{general_files.module_tree.to_ruby_classes}
  RUBY

  declaration_files = Yass::Codegen::RustFileSet.new(Dir.glob("ext/yass/src/declarations/**/*.rs"))

  File.write("lib/yass/declarations.rb", <<~RUBY)
    # frozen_string_literal: true

    #{declaration_files.module_tree.to_ruby_classes}
  RUBY

  selector_files = Yass::Codegen::RustFileSet.new(Dir.glob("ext/yass/src/selectors/**/*.rs"))

  File.write("lib/yass/selectors.rb", <<~RUBY)
    # frozen_string_literal: true

    #{selector_files.module_tree.to_ruby_classes}
  RUBY

  rule_files = Yass::Codegen::RustFileSet.new([
    "ext/yass/src/rules/rule.rs",
    "ext/yass/src/rules/style_rule.rs",
    "ext/yass/src/rules/media_rule.rs",
    "ext/yass/src/rules/font_face_rule.rs",
    *Dir.glob("ext/yass/src/rules/fonts/**/*.rs"),
  ])

  File.write("lib/yass/rules.rb", <<~RUBY)
    # frozen_string_literal: true

    #{rule_files.module_tree.to_ruby_classes}
  RUBY

  all_modules = general_files.module_tree
    .merge(declaration_files.module_tree)
    .merge(selector_files.module_tree)
    .merge(rule_files.module_tree)

  visitor_class = <<~RUBY
    # frozen_string_literal: true

    module Yass
      class Visitor
        def visit(node)
          node.accept(self) if node
        end

        def visit_list(nodes)
          nodes.each { |node| visit(node) }
        end

        def visit_stylesheet(node)
          visit_list(node.rules)
        end

    #{all_modules.to_visitor_methods(indent: "    ").join("\n")}
      end
    end
  RUBY

  File.write("lib/yass/visitor.rb", visitor_class)
end
