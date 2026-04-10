# frozen_string_literal: true

require "spec_helper"

RSpec.describe "text-align-last declaration" do
  def declaration_for(value)
    sheet = Yass::Parser.parse(".x { text-align-last: #{value}; }")
    sheet.rules.first.declarations.first
  end

  it "exposes text-align-last values" do
    {
      "auto" => :auto,
      "start" => :start,
      "end" => :end,
      "left" => :left,
      "right" => :right,
      "center" => :center,
      "justify" => :justify,
    }.each do |css_value, expected_value|
      declaration = declaration_for(css_value)

      expect(declaration).to be_a(Yass::Declarations::TextAlignLast)
      expect(declaration.value).to eq(expected_value)
    end
  end
end
