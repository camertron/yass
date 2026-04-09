# frozen_string_literal: true

require "spec_helper"

RSpec.describe "line-break declaration" do
  def declaration_for(property, value)
    sheet = Yass::Parser.parse(".x { #{property}: #{value}; }")
    sheet.rules.first.declarations.first
  end

  it "exposes line-break values" do
    {
      "auto" => :auto,
      "loose" => :loose,
      "normal" => :normal,
      "strict" => :strict,
      "anywhere" => :anywhere,
    }.each do |css_value, expected_value|
      declaration = declaration_for("line-break", css_value)

      expect(declaration).to be_a(Yass::Declarations::LineBreak)
      expect(declaration.value).to eq(expected_value)
    end
  end
end
