# frozen_string_literal: true

require_relative "utils"

module Yass
  module Codegen
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

    class RubyModuleTree
      class << self
        def from_structs(rust_structs)
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

          new(root)
        end
      end

      include Utils

      attr_reader :root

      def initialize(root)
        @root = root
      end

      def to_ruby_classes(indent: "")
        to_ruby_classes_helper(root.children["Yass"], ["Yass"], indent)
      end

      def to_visitor_methods(indent:)
        to_visitor_methods_helper(root.children["Yass"], ["Yass"], indent)
      end

      def merge(other)
        if root.name != other.root.name
          raise ArgumentError, "this tree and other tree have different names, '#{name}' vs '#{other.root.name}'"
        end

        new_root = merge_helper(root, other.root)
        self.class.new(new_root)
      end

      private

      def merge_helper(first, second)
        RubyModule.new(first.name).tap do |mod|
          first.children.each do |key, first_child_mod|
            if (second_child_mod = second.children[key])
              mod.children[key] = merge_helper(first_child_mod, second_child_mod)
            else
              mod.children[key] = first_child_mod
            end
          end

          second.children.each do |key, second_child_mod|
            # merge for identical keys should happen in first pass above
            if !first.children.include?(key)
              mod.children[key] = second_child_mod
            end
          end
        end
      end

      def to_ruby_classes_helper(mod, nesting, indent = "")
        class_or_module = Kernel.const_get(nesting.join("::"))
        class_or_module_def = class_or_module.class == Class ? "class #{mod.name}" : "module #{mod.name}"

        children = mod.children.map do |mod_name, child|
          to_ruby_classes_helper(child, [*nesting, mod_name], indent + "  ")
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
              "#{indent}  RUBY_METHODS = %i(#{method_names.map(&:to_s).sort.join(" ")}).freeze"
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

      def to_visitor_methods_helper(mod, nesting, indent)
        line_groups = []

        if mod.rust_struct
          method_name = "visit_#{nesting_to_s(nesting[1..-1])}"

          line_groups << [
            "#{indent}def #{method_name}(node)",
            *(debug? ? ["#{indent}  puts \"Visiting #{method_name}\""] : []),
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
          ]
        end

        mod.children.each do |mod_name, child|
          line_groups << to_visitor_methods_helper(child, [*nesting, mod_name], indent)
        end

        join_line_groups(*line_groups)
      end

      def debug?
        !!ENV["DEBUG"]
      end
    end
  end
end
