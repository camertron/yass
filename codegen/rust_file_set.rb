# frozen_string_literal: true

require_relative "ruby_module_tree"

module Yass
  module Codegen
    class RustStruct
      attr_reader :name, :ruby_class_name, :rust_methods

      def initialize(name:, ruby_class_name:, rust_methods:)
        @name = name
        @ruby_class_name = ruby_class_name
        @rust_methods = rust_methods
      end
    end

    class RustMethod
      attr_reader :name, :return_type, :visit_kind

      def initialize(name:, return_type:, visit_kind:)
        @name = name
        @return_type = return_type
        @visit_kind = visit_kind
      end
    end

    class RustFileSet
      attr_reader :rust_file_paths

      def initialize(rust_file_paths)
        @rust_file_paths = rust_file_paths
      end

      def structs
        @structs ||= rust_file_paths.flat_map do |rust_file|
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
                rust_methods << RustMethod.new(name: name, return_type: return_type, visit_kind: :single)
              elsif ["RArray", "Result<RArray, Error>"].include?(return_type)
                rust_methods << RustMethod.new(name: name, return_type: return_type, visit_kind: :multiple)
              end
            end

            RustStruct.new(
              name: rust_struct_name,
              ruby_class_name: ruby_class_name,
              rust_methods: rust_methods
            )
          end
        end
      end

      def module_tree
        @module_tree ||= RubyModuleTree.from_structs(structs)
      end
    end
  end
end
