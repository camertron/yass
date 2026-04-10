# frozen_string_literal: true

require "spec_helper"

RSpec.describe(Yass) do
  describe "pointer-events declarations" do
    def pointer_events_declaration(value)
      sheet = Yass::Parser.parse(".x { pointer-events: #{value}; }")
      sheet.rules.first.declarations.first
    end

    it "exposes pointer-events values as declaration wrappers" do
      {
        "auto" => :auto,
        "none" => :none,
      }.each do |css_value, expected_value|
        declaration = pointer_events_declaration(css_value)

        expect(declaration).to be_a(Yass::Declarations::PointerEvents)
        expect(declaration.value).to eq(expected_value)
      end
    end
  end
end
