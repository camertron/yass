# frozen_string_literal: true

module Yass
  class UnicodeRange
    RUBY_METHODS = %i(end kind start).freeze

    include ::Yass::Node

    def accept(visitor)
      visitor.visit_unicode_range(self)
    end

    def kind
      :unicode_range
    end
  end

  class SourceLocation
    RUBY_METHODS = %i(column kind line).freeze

    include ::Yass::Node

    def accept(visitor)
      visitor.visit_source_location(self)
    end

    def kind
      :source_location
    end
  end
end
