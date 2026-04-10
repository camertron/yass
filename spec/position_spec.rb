# frozen_string_literal: true

require "spec_helper"

RSpec.describe "position declaration" do
  def declaration_for(value)
    sheet = Yass::Parser.parse(".x { position: #{value}; }")
    sheet.rules.first.declarations.first
  end

  it "exposes position values" do
    {
      "static" => :static,
      "relative" => :relative,
      "absolute" => :absolute,
      "fixed" => :fixed,
      "sticky" => :sticky,
    }.each do |css_value, expected_value|
      declaration = declaration_for(css_value)

      expect(declaration).to be_a(Yass::Declarations::Position)
      expect(declaration.value).to eq(expected_value)
    end
  end
end
