# frozen_string_literal: true

module Yass
  module Codegen
    module Utils
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
    end
  end
end
