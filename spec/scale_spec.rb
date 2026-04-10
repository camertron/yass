require_relative "spec_helper"

RSpec.describe(Yass) do
  describe "scale declarations" do
    it "exposes none value" do
      stylesheet = Yass::Parser.parse("div { scale: none; }")
      decl = stylesheet.rules[0].declarations[0]

      expect(decl).to be_a(Yass::Declarations::Scale)
      expect(decl.value).to be_a(Yass::Declarations::Scale::None)
    end

    it "exposes scale x component" do
      stylesheet = Yass::Parser.parse("div { scale: 2; }")
      decl = stylesheet.rules[0].declarations[0]
      value = decl.value

      expect(value).to be_a(Yass::Declarations::Scale::Coords)
      expect(value.x.value).to eq(2.0)
    end

    it "exposes scale x/y components" do
      stylesheet = Yass::Parser.parse("div { scale: 2 3; }")
      decl = stylesheet.rules[0].declarations[0]
      value = decl.value

      expect(value).to be_a(Yass::Declarations::Scale::Coords)
      expect(value.x.value).to eq(2.0)
      expect(value.y.value).to eq(3.0)
    end

    it "exposes scale x/y/z components" do
      stylesheet = Yass::Parser.parse("div { scale: 2 3 4; }")
      decl = stylesheet.rules[0].declarations[0]
      value = decl.value

      expect(value).to be_a(Yass::Declarations::Scale::Coords)
      expect(value.x.value).to eq(2.0)
      expect(value.y.value).to eq(3.0)
      expect(value.z.value).to eq(4.0)
    end
  end
end
