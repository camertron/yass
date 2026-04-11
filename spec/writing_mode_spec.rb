# frozen_string_literal: true

require "spec_helper"

RSpec.describe "writing-mode declaration" do
  def declaration_for(value)
    sheet = Yass::Parser.parse(".x { writing-mode: #{value}; }")
    sheet.rules.first.declarations.first
  end

  it "exposes writing-mode values" do
    {
      "horizontal-tb" => :horizontal_tb,
      "vertical-rl"   => :vertical_rl,
      "vertical-lr"   => :vertical_lr,
    }.each do |css_value, expected_value|
      declaration = declaration_for(css_value)

      expect(declaration).to be_a(Yass::Declarations::WritingMode)
      expect(declaration.value).to eq(expected_value)
    end
  end
end
