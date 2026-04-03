require "yass"

RustStruct = Struct.new(:name, :ruby_class_name, :rust_methods)
RustMethod = Struct.new(:name, :return_type, :visit_kind)

def extract_rust_structs_from(rust_files)
  rust_files.flat_map do |rust_file|
    rust_code = File.read(rust_file)

    rust_code.scan(/#\[magnus(?:\:\:wrap)?\(class = "([\w:]+)"(?:, mark)?\)\]\s+pub struct (\w+)/).map do |ruby_class_name, rust_struct_name|
      next unless ruby_class_name && rust_struct_name

      m = rust_code.match(/impl\s+#{rust_struct_name}\s+(\{)/)
      next unless m

      start = pos = m.end(1)
      count = 1

      while count > 0
        case rust_code[pos]
        when "{"
          count += 1
        when "}"
          count -= 1
        end

        pos += 1
      end

      impl_body = rust_code[start..pos]
      rust_method_names = impl_body.scan(/pub fn (\w+)\(ruby: &Ruby, rb_self: typed_data::Obj<Self>\) -> ([\w<>:, ]+) \{/)
      rust_methods = []

      rust_method_names.each do |name, return_type|
        if ["Option<Value>", "Value"].include?(return_type)
          rust_methods << RustMethod.new(name, return_type, :single)
        elsif ["RArray", "Result<RArray, Error>"].include?(return_type)
          rust_methods << RustMethod.new(name, return_type, :multiple)
        end
      end

      RustStruct.new(rust_struct_name, ruby_class_name, rust_methods)
    end
  end
end

class RubyModule
  attr_reader :name
  attr_accessor :rust_struct

  def initialize(name)
    @name = name
  end

  def children
    @children ||= {}
  end
end

def module_tree_from_structs(rust_structs)
  root = RubyModule.new(nil)

  rust_structs.each do |rust_struct|
    current = root

    rust_struct.ruby_class_name.split("::").each do |ns|
      if !current.children.include?(ns)
        current.children[ns] = RubyModule.new(ns)
      end

      current = current.children[ns]
    end

    current.rust_struct = rust_struct
  end

  root
end

declaration_files = Dir.glob("ext/yass/src/declarations/**/*.rs")
declaration_rust_structs = extract_rust_structs_from(declaration_files)
root = module_tree_from_structs(declaration_rust_structs)

def underscore(str)
  str.gsub(/(?<=[A-Z])(?=[A-Z][a-z])|(?<=[a-z\d])(?=[A-Z])/, "_")
end

def nesting_to_s(nesting)
  nesting
    .map { |part| underscore(part).downcase }
    .map { |part| part == "declarations" ? "declaration" : part }
    .join("_")
end

def join_line_groups(*line_groups)
  result = line_groups.compact.reject(&:empty?).flat_map do |group|
    [*group.flatten, ""]
  end

  result.pop
  result
end

def module_to_code(mod, nesting, indent = "")
  class_or_module = Kernel.const_get(nesting.join("::"))
  class_or_module_def = class_or_module.class == Class ? "class #{mod.name}" : "module #{mod.name}"

  children = mod.children.map do |mod_name, child|
    module_to_code(child, [*nesting, mod_name], indent + "  ")
  end

  accept_method = if mod.rust_struct
    [
      "#{indent}  def accept(visitor)",
      "#{indent}    visitor.visit_#{nesting_to_s(nesting[1..-1])}(self)",
      "#{indent}  end",
    ]
  end

  kind_method = if mod.rust_struct
    [
      "#{indent}  def kind",
      "#{indent}    :#{underscore(nesting.last).downcase}",
      "#{indent}  end",
    ]
  end

  constants = if mod.rust_struct
    method_names = class_or_module.instance_methods - Object.methods - [:accept, :to_h]

    if method_names.size > 0
      [
        "#{indent}  RUBY_METHODS = %i(#{method_names.map(&:to_s).join(" ")}).freeze"
      ]
    end
  end

  includes = if mod.rust_struct
    ["#{indent}  include ::Yass::Node"]
  end

  lines = [
    "#{indent}#{class_or_module_def}",
    *join_line_groups(
      constants,
      includes,
      accept_method,
      kind_method,
      (children.empty? ? [] : [children.join("\n\n")]),
    ),
    "#{indent}end",
  ]

  lines.join("\n")
end

declaration_classes = module_to_code(root.children["Yass"], ["Yass"])

File.write("lib/yass/declarations.rb", <<~RUBY)
  # frozen_string_literal: true

  #{declaration_classes}
RUBY

def module_to_visitor_methods(mod, nesting, indent)
  [].tap do |lines|
    if mod.rust_struct
      method_name = "visit_#{nesting_to_s(nesting[1..-1])}"

      lines.concat([
        "#{indent}def #{method_name}(node)",
        *mod.rust_struct.rust_methods.map do |rust_method|
          if rust_method.visit_kind == :single
            [
              "#{indent}  visit(node.#{rust_method.name})"
            ]
          else
            [
              "#{indent}  visit_list(node.#{rust_method.name})"
            ]
          end
        end,
        "#{indent}end",
      ])
    end

    visitor_methods = mod.children.map do |mod_name, child|
      module_to_visitor_methods(child, [*nesting, mod_name], indent)
    end

    lines.concat(join_line_groups(visitor_methods))
  end
end

visitor_methods = module_to_visitor_methods(root.children["Yass"], ["Yass"], "    ")

visitor_class = <<~RUBY
  # frozen_string_literal: true

  module Yass
    class Visitor
      def visit(node)
        node.accept(self)
      end

      def visit_list(nodes)
        nodes.each { |node| visit(node) }
      end

  #{visitor_methods.join("\n")}
    end
  end
RUBY

File.write("lib/yass/visitor.rb", visitor_class)

selector_files = Dir.glob("ext/yass/src/selectors/**/*.rs")
selector_rust_structs = extract_rust_structs_from(selector_files)
root = module_tree_from_structs(selector_rust_structs)
selector_classes = module_to_code(root.children["Yass"], ["Yass"])

File.write("lib/yass/selectors.rb", <<~RUBY)
  # frozen_string_literal: true

  #{selector_classes}
RUBY
