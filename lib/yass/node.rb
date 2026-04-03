# frozen_string_literal

module Yass
  module Node
    def to_h
      Yass.serialize(self)
    end
  end
end
