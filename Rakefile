# frozen_string_literal: true

require "bundler/gem_tasks"
require "rspec/core/rake_task"

RSpec::Core::RakeTask.new(:spec)

require "rb_sys/extensiontask"

task build: :compile

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
