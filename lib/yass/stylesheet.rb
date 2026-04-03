# frozen_string_literal: true

module Yass
  class Stylesheet
    def to_h
      {
        rules: rules.map do |rule|
          {
            selectors: rule.selectors.map(&:to_h),
            declarations: rule.declarations.map(&:to_h)
          }
        end
      }
    end
  end
end
