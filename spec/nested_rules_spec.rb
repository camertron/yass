require_relative "spec_helper"

RSpec.describe(Yass) do
  describe "nested rules" do
    it "exposes nested rules" do
      stylesheet = Yass::Parser.parse(".foo { .bar { float:left; } }")
      nested_rules = stylesheet.rules[0].rules

      expect(nested_rules.size).to eq(1)
      expect(nested_rules.first.declarations.size).to eq(1)
      expect(nested_rules.first.declarations.first).to be_a(Yass::Declarations::Float)
    end
  end
end
