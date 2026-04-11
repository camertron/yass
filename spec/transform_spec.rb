require_relative "spec_helper"

RSpec.describe(Yass) do
  describe "transform declarations" do
    it "exposes none as an empty operation list" do
      stylesheet = Yass::Parser.parse("div { transform: none; }")
      decl = stylesheet.rules[0].declarations[0]

      expect(decl).to be_a(Yass::Declarations::Transform)
      expect(decl.values).to eq([])
    end

    it "exposes matrix components" do
      stylesheet = Yass::Parser.parse("div { transform: matrix(1, 2, 3, 4, 5, 6); }")
      value = stylesheet.rules[0].declarations[0].values[0]

      expect(value).to be_a(Yass::Declarations::Transform::Matrix)
      expect(value.a.value).to eq(1.0)
      expect(value.f.value).to eq(6.0)
    end

    it "exposes translate3d components" do
      stylesheet = Yass::Parser.parse("div { transform: translate3d(10px, 20%, 30px); }")
      value = stylesheet.rules[0].declarations[0].values[0]

      expect(value).to be_a(Yass::Declarations::Transform::Translate3D)
      expect(value.x.value).to be_a(Yass::Declarations::Length::Absolute)
      expect(value.y.value).to be_a(Yass::Declarations::Percentage)
      expect(value.z).to be_a(Yass::Declarations::Length::Absolute)
    end

    it "exposes perspective length" do
      stylesheet = Yass::Parser.parse("div { transform: perspective(400px); }")
      value = stylesheet.rules[0].declarations[0].values[0]

      expect(value).to be_a(Yass::Declarations::Transform::Perspective)
      expect(value.value).to be_a(Yass::Declarations::Transform::Perspective::Length)
      expect(value.value.value).to be_a(Yass::Declarations::Length::Absolute)
    end
  end
end
