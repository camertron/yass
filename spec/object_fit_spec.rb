require_relative "spec_helper"

RSpec.describe(Yass) do
  describe "object-fit declarations" do
    it "exposes fill keyword" do
      stylesheet = Yass::Parser.parse("div { object-fit: fill; }")
      decl = stylesheet.rules[0].declarations[0]
      expect(decl).to be_a(Yass::Declarations::ObjectFit)
      expect(decl.value).to eq(:fill)
    end

    it "exposes contain keyword" do
      stylesheet = Yass::Parser.parse("div { object-fit: contain; }")
      decl = stylesheet.rules[0].declarations[0]
      expect(decl.value).to eq(:contain)
    end

    it "exposes cover keyword" do
      stylesheet = Yass::Parser.parse("div { object-fit: cover; }")
      decl = stylesheet.rules[0].declarations[0]
      expect(decl.value).to eq(:cover)
    end

    it "exposes scale-down keyword" do
      stylesheet = Yass::Parser.parse("div { object-fit: scale-down; }")
      decl = stylesheet.rules[0].declarations[0]
      expect(decl.value).to eq(:scale_down)
    end

    it "exposes none keyword" do
      stylesheet = Yass::Parser.parse("div { object-fit: none; }")
      decl = stylesheet.rules[0].declarations[0]
      expect(decl.value).to eq(:none)
    end
  end
end
