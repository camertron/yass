# frozen_string_literal: true

require "spec_helper"

RSpec.describe "-webkit-text-security declaration" do
  def declaration_for(value)
    sheet = Yass::Parser.parse(".x { -webkit-text-security: #{value}; }")
    sheet.rules.first.declarations.first
  end

  it "exposes -webkit-text-security values" do
    {
      "none" => :none,
      "circle" => :circle,
      "disc" => :disc,
      "square" => :square,
    }.each do |css_value, expected_value|
      declaration = declaration_for(css_value)

      expect(declaration).to be_a(Yass::Declarations::WebkitTextSecurity)
      expect(declaration.value).to eq(expected_value)
    end
  end
end
