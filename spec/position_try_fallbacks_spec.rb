# frozen_string_literal: true

require "spec_helper"

RSpec.describe(Yass) do
  describe "position-try-fallbacks declarations" do
    def position_try_fallbacks_declaration(value)
      sheet = Yass::Parser.parse(".x { position-try-fallbacks: #{value}; }")
      sheet.rules.first.declarations.first
    end

    it "exposes none as empty values" do
      declaration = position_try_fallbacks_declaration("none")

      expect(declaration).to be_a(Yass::Declarations::PositionTryFallbacks)
      expect(declaration.is_none).to eq(true)
      expect(declaration.values).to eq([])
    end

    it "exposes fallback items as wrapped variants" do
      declaration = position_try_fallbacks_declaration("--foo flip-block, right top")

      expect(declaration).to be_a(Yass::Declarations::PositionTryFallbacks)
      expect(declaration.is_none).to eq(false)
      expect(declaration.values.length).to eq(2)

      ident_and_or_tactic = declaration.values[0]
      expect(ident_and_or_tactic).to be_a(Yass::Declarations::PositionTryFallbacks::IdentAndOrTactic)
      expect(ident_and_or_tactic.has_ident?).to eq(true)
      expect(ident_and_or_tactic.ident).to eq("--foo")
      expect(ident_and_or_tactic.has_try_tactics?).to eq(true)
      expect(ident_and_or_tactic.try_tactics).to eq([:flip_block])

      position_area = declaration.values[1]
      expect(position_area).to be_a(Yass::Declarations::PositionTryFallbacks::PositionArea)
      expect(position_area.first).to eq(:right)
      expect(position_area.second).to eq(:top)
    end

    it "returns nil for missing ident" do
      declaration = position_try_fallbacks_declaration("flip-inline")
      ident_and_or_tactic = declaration.values.first

      expect(ident_and_or_tactic).to be_a(Yass::Declarations::PositionTryFallbacks::IdentAndOrTactic)
      expect(ident_and_or_tactic.has_ident?).to eq(false)
      expect(ident_and_or_tactic.ident).to eq(nil)
      expect(ident_and_or_tactic.try_tactics).to eq([:flip_inline])
    end
  end
end
