# frozen_string_literal: true

module Yass
  class UnicodeRange
    RUBY_METHODS = %i(end start).freeze

    include ::Yass::Node

    def accept(visitor)
      visitor.visit_unicode_range(self)
    end

    def kind
      :unicode_range
    end
  end
end
