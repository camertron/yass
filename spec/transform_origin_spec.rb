require_relative "spec_helper"

RSpec.describe(Yass) do
  describe "transform-origin declarations" do
    it "exposes horizontal, vertical, and depth components for keyword values" do
      stylesheet = Yass::Parser.parse("div { transform-origin: left top 5px; }")
      decl = stylesheet.rules[0].declarations[0]

      expect(decl).to be_a(Yass::Declarations::TransformOrigin)
      expect(decl.horizontal).to be_a(Yass::Declarations::TransformOrigin::SideHorizontalComponent)
      expect(decl.horizontal.side).to eq(:left)
      expect(decl.vertical).to be_a(Yass::Declarations::TransformOrigin::SideVerticalComponent)
      expect(decl.vertical.side).to eq(:top)
      expect(decl.depth).not_to be_nil
    end

    it "exposes length percentage components" do
      stylesheet = Yass::Parser.parse("div { transform-origin: 25% 75%; }")
      decl = stylesheet.rules[0].declarations[0]

      expect(decl).to be_a(Yass::Declarations::TransformOrigin)
      expect(decl.horizontal).to be_a(Yass::Declarations::Size::LengthPercentage)
      expect(decl.vertical).to be_a(Yass::Declarations::Size::LengthPercentage)
      expect(decl.depth).not_to be_nil
    end

    it "exposes center components" do
      stylesheet = Yass::Parser.parse("div { transform-origin: center; }")
      decl = stylesheet.rules[0].declarations[0]

      expect(decl).to be_a(Yass::Declarations::TransformOrigin)
      expect(decl.horizontal).to be_a(Yass::Declarations::TransformOrigin::Center)
      expect(decl.vertical).to be_a(Yass::Declarations::TransformOrigin::Center)
    end
  end
end
