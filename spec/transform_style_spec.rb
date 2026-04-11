require_relative "spec_helper"

RSpec.describe(Yass) do
  describe "transform-style declarations" do
    it "exposes flat keyword" do
      stylesheet = Yass::Parser.parse("div { transform-style: flat; }")
      decl = stylesheet.rules[0].declarations[0]

      expect(decl).to be_a(Yass::Declarations::TransformStyle)
      expect(decl.value).to eq(:flat)
    end

    it "exposes preserve-3d keyword" do
      stylesheet = Yass::Parser.parse("div { transform-style: preserve-3d; }")
      decl = stylesheet.rules[0].declarations[0]

      expect(decl).to be_a(Yass::Declarations::TransformStyle)
      expect(decl.value).to eq(:preserve_3d)
    end
  end
end
