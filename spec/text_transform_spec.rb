# frozen_string_literal: true

require "spec_helper"

RSpec.describe "text-transform declaration" do
  def declaration_for(value)
    sheet = Yass::Parser.parse(".x { text-transform: #{value}; }")
    sheet.rules.first.declarations.first
  end

  it "exposes text-transform values" do
    {
      "none" => [:none],
      "capitalize" => [:capitalize],
      "uppercase" => [:uppercase],
      "lowercase" => [:lowercase],
      "full-width" => [:full_width],
      "full-size-kana" => [:full_size_kana],
    }.each do |css_value, expected_values|
      declaration = declaration_for(css_value)

      expect(declaration).to be_a(Yass::Declarations::TextTransform)
      expect(declaration.values).to eq(expected_values)
    end
  end

  it "exposes combinable text-transform values" do
    declaration = declaration_for("capitalize full-width")

    expect(declaration).to be_a(Yass::Declarations::TextTransform)
    expect(declaration.values).to match_array([:capitalize, :full_width])
  end
end
