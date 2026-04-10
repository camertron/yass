require_relative "spec_helper"

RSpec.describe(Yass) do
  describe "rotate declarations" do
    it "exposes none value" do
      stylesheet = Yass::Parser.parse("div { rotate: none; }")
      decl = stylesheet.rules[0].declarations[0]

      expect(decl).to be_a(Yass::Declarations::Rotate)
      expect(decl.value).to be_a(Yass::Declarations::Rotate::None)
    end

    it "exposes angle value" do
      stylesheet = Yass::Parser.parse("div { rotate: 45deg; }")
      decl = stylesheet.rules[0].declarations[0]

      expect(decl.value).to be_a(Yass::Declarations::Angle)
      expect(decl.value.degrees).to eq(45.0)
    end

    it "exposes rotate3d components" do
      stylesheet = Yass::Parser.parse("div { rotate: 1 0 0 30deg; }")
      decl = stylesheet.rules[0].declarations[0]
      value = decl.value

      expect(value).to be_a(Yass::Declarations::Rotate::Rotate3D)
      expect(value.x.value).to eq(1.0)
      expect(value.y.value).to eq(0.0)
      expect(value.z.value).to eq(0.0)
      expect(value.angle.degrees).to eq(30.0)
    end
  end
end
