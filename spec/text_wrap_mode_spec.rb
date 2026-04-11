# frozen_string_literal: true

require "spec_helper"

RSpec.describe "text-wrap-mode declaration" do
  def declaration_for(value)
    sheet = Yass::Parser.parse(".x { text-wrap-mode: #{value}; }")
    sheet.rules.first.declarations.first
  end

  it "exposes text-wrap-mode values" do
    {
      "wrap" => :wrap,
      "nowrap" => :nowrap,
    }.each do |css_value, expected_value|
      declaration = declaration_for(css_value)

      expect(declaration).to be_a(Yass::Declarations::TextWrapMode)
      expect(declaration.value).to eq(expected_value)
    end
  end
end
