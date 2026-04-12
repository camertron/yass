# frozen_string_literal: true

require_relative "yass/version"

# Tries to require the precompiled extension for the given Ruby version first
begin
  RUBY_VERSION =~ /(\d+\.\d+)/
  require "yass/#{Regexp.last_match(1)}/yass"
rescue LoadError
  require_relative "yass/yass"
end

require "yass/node"
require "yass/stylesheet"

require "yass/declarations"
require "yass/rules"
require "yass/selectors"

module Yass
  autoload :Visitor, "yass/visitor"

  class Stylesheet
    def accept(visitor)
      visitor.visit_stylesheet(self)
    end
  end

  class StyleRule
    def accept(visitor)
      visitor.visit_style_rule(self)
    end
  end

  class UnimplementedRule
    def accept(visitor)
    end

    def selectors
      []
    end

    def declarations
      []
    end
  end

  def self.serialize(obj)
    case obj
    when Integer, String, Symbol
      obj
    when Float
      if obj == Float::INFINITY
        "inf"
      elsif obj == -Float::INFINITY
        "-inf"
      elsif obj.nan?
        "NaN"
      else
        obj
      end
    when Array
      obj.map { |elem| serialize(elem) }
    else
      if obj.class.const_defined?(:RUBY_METHODS)
        obj.class::RUBY_METHODS.each_with_object({}) do |method_name, memo|
          plain_method_name = method_name.to_s.chomp("?")
          memo[plain_method_name.to_sym] = serialize(obj.send(method_name))
        end
      end
    end
  end
end
