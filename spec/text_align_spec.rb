# frozen_string_literal: true

require "spec_helper"

RSpec.describe "text-align declaration" do
  def declaration_for(value)
    sheet = Yass::Parser.parse(".x { text-align: #{value}; }")
    sheet.rules.first.declarations.first
  end

  it "exposes keyword values" do
    {
      "start" => :start,
      "end" => :end,
      "left" => :left,
      "right" => :right,
      "center" => :center,
      "justify" => :justify,
    }.each do |css_value, expected_value|
      declaration = declaration_for(css_value)

      expect(declaration).to be_a(Yass::Declarations::TextAlign)
      expect(declaration.value).to eq(expected_value)
    end
  end

  it "maps webkit aliases to moz-prefixed symbols" do
    {
      "-webkit-center" => :moz_center,
      "-webkit-left" => :moz_left,
      "-webkit-right" => :moz_right,
    }.each do |css_value, expected_value|
      declaration = declaration_for(css_value)

      expect(declaration).to be_a(Yass::Declarations::TextAlign)
      expect(declaration.value).to eq(expected_value)
    end
  end
end
