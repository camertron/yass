require_relative "spec_helper"

RSpec.describe(Yass) do
  describe "quotes declarations" do
    it "exposes auto as a nested wrapper" do
      stylesheet = Yass::Parser.parse("div { quotes: auto; }")
      decl = stylesheet.rules[0].declarations[0]

      expect(decl).to be_a(Yass::Declarations::Quotes)
      expect(decl.value).to be_a(Yass::Declarations::Quotes::Auto)
    end

    it "exposes none as a nested wrapper" do
      stylesheet = Yass::Parser.parse("div { quotes: none; }")
      decl = stylesheet.rules[0].declarations[0]

      expect(decl.value).to be_a(Yass::Declarations::Quotes::None)
    end

    it "exposes explicit quote pairs" do
      stylesheet = Yass::Parser.parse("div { quotes: \"<\" \">\" \"[\" \"]\"; }")
      decl = stylesheet.rules[0].declarations[0]
      value = decl.value

      expect(value).to be_a(Yass::Declarations::Quotes::QuoteList)
      expect(value.values.map { |pair| [pair.opening, pair.closing] }).to eq([
        ["<", ">"],
        ["[", "]"]
      ])
    end
  end
end
