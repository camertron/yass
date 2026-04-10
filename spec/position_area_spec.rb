# frozen_string_literal: true

require "spec_helper"

RSpec.describe(Yass) do
  describe "position-area declarations" do
    def position_area_declaration(value)
      sheet = Yass::Parser.parse(".x { position-area: #{value}; }")
      sheet.rules.first.declarations.first
    end

    it "exposes position-area first and second keywords as symbols" do
      declaration = position_area_declaration("left top")
      expect(declaration).to be_a(Yass::Declarations::PositionArea)
      expect(declaration.first).to eq(:left)
      expect(declaration.second).to eq(:top)
    end

    it "exposes is_none as true for the none value" do
      declaration = position_area_declaration("none")
      expect(declaration).to be_a(Yass::Declarations::PositionArea)
      expect(declaration.is_none).to eq(true)
    end

    it "exposes is_none as false for non-none values" do
      declaration = position_area_declaration("center")
      expect(declaration).to be_a(Yass::Declarations::PositionArea)
      expect(declaration.is_none).to eq(false)
    end
  end
end
