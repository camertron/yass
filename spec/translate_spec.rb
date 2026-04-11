require_relative "spec_helper"

RSpec.describe(Yass) do
  describe "translate declarations" do
    it "exposes none value" do
      stylesheet = Yass::Parser.parse("div { translate: none; }")
      decl = stylesheet.rules[0].declarations[0]

      expect(decl).to be_a(Yass::Declarations::Translate)
      expect(decl.value).to be_a(Yass::Declarations::Translate::None)
    end

    it "exposes single length x component" do
      stylesheet = Yass::Parser.parse("div { translate: 10px; }")
      decl = stylesheet.rules[0].declarations[0]
      value = decl.value

      expect(value).to be_a(Yass::Declarations::Translate::Coords)
      expect(value.x).to be_a(Yass::Declarations::Length::Absolute)
      expect(value.x.value).to eq(10.0)
    end

    it "exposes x/y length and percentage components" do
      stylesheet = Yass::Parser.parse("div { translate: 10px 20%; }")
      decl = stylesheet.rules[0].declarations[0]
      value = decl.value

      expect(value).to be_a(Yass::Declarations::Translate::Coords)
      expect(value.x).to be_a(Yass::Declarations::Length::Absolute)
      expect(value.x.value).to eq(10.0)
      expect(value.y).to be_a(Yass::Declarations::Percentage)
      expect(value.y.value).to be_within(0.0001).of(0.2)
    end

    it "exposes x/y/z components" do
      stylesheet = Yass::Parser.parse("div { translate: 10px 20% 30px; }")
      decl = stylesheet.rules[0].declarations[0]
      value = decl.value

      expect(value).to be_a(Yass::Declarations::Translate::Coords)
      expect(value.x).to be_a(Yass::Declarations::Length::Absolute)
      expect(value.x.value).to eq(10.0)
      expect(value.y).to be_a(Yass::Declarations::Percentage)
      expect(value.y.value).to be_within(0.0001).of(0.2)
      expect(value.z).to be_a(Yass::Declarations::Length::Absolute)
      expect(value.z.value).to eq(30.0)
    end
  end
end
