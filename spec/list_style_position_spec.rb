# frozen_string_literal: true

require "spec_helper"

RSpec.describe "list-style-position declaration" do
  def declaration_for(value)
    sheet = Yass::Parser.parse(".x { list-style-position: #{value}; }")
    sheet.rules.first.declarations.first
  end

  it "exposes list-style-position values" do
    {
      "inside" => :inside,
      "outside" => :outside,
    }.each do |css_value, expected_value|
      declaration = declaration_for(css_value)

      expect(declaration).to be_a(Yass::Declarations::ListStylePosition)
      expect(declaration.value).to eq(expected_value)
    end
  end
end
