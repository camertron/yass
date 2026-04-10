# frozen_string_literal: true

require "spec_helper"

RSpec.describe "text-decoration-style declaration" do
  def declaration_for(value)
    sheet = Yass::Parser.parse(".x { text-decoration-style: #{value}; }")
    sheet.rules.first.declarations.first
  end

  it "exposes keyword values" do
    {
      "solid" => :solid,
      "double" => :double,
      "dotted" => :dotted,
      "dashed" => :dashed,
      "wavy" => :wavy,
    }.each do |css_value, expected_value|
      declaration = declaration_for(css_value)

      expect(declaration).to be_a(Yass::Declarations::TextDecorationStyle)
      expect(declaration.value).to eq(expected_value)
    end
  end
end
