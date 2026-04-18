# frozen_string_literal: true

module Yass
  class Stylesheet
    def to_h
      { rules: rules.map(&:to_h) }
    end
  end
end
