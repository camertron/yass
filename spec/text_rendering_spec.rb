# frozen_string_literal: true

require "spec_helper"

RSpec.describe "text-rendering declaration" do
  def declaration_for(value)
    sheet = Yass::Parser.parse(".x { text-rendering: #{value}; }")
    sheet.rules.first.declarations.first
  end

  it "exposes text-rendering values" do
    {
      "auto" => :auto,
      "optimizespeed" => :optimize_speed,
      "optimizelegibility" => :optimize_legibility,
      "geometricprecision" => :geometric_precision,
    }.each do |css_value, expected_value|
      declaration = declaration_for(css_value)

      expect(declaration).to be_a(Yass::Declarations::TextRendering)
      expect(declaration.value).to eq(expected_value)
    end
  end
end
