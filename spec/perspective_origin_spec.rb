require_relative "spec_helper"

RSpec.describe(Yass) do
  describe "perspective-origin declarations" do
    it "exposes horizontal and vertical position components" do
      stylesheet = Yass::Parser.parse("div { perspective-origin: 25% 75%; }")
      decl = stylesheet.rules[0].declarations[0]

      expect(decl).to be_a(Yass::Declarations::PerspectiveOrigin)
      expect(decl.horizontal).not_to be_nil
      expect(decl.vertical).not_to be_nil
    end

    it "exposes keyword-based position components" do
      stylesheet = Yass::Parser.parse("div { perspective-origin: left top; }")
      decl = stylesheet.rules[0].declarations[0]

      expect(decl.horizontal).not_to be_nil
      expect(decl.vertical).not_to be_nil
    end
  end
end
