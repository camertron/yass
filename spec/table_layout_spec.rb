# frozen_string_literal: true

require "spec_helper"

RSpec.describe "table-layout declaration" do
  def declaration_for(value)
    sheet = Yass::Parser.parse(".x { table-layout: #{value}; }")
    sheet.rules.first.declarations.first
  end

  it "exposes table-layout values" do
    {
      "auto" => :auto,
      "fixed" => :fixed,
    }.each do |css_value, expected_value|
      declaration = declaration_for(css_value)

      expect(declaration).to be_a(Yass::Declarations::TableLayout)
      expect(declaration.value).to eq(expected_value)
    end
  end
end
