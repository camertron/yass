# frozen_string_literal: true

require "spec_helper"

RSpec.describe "white-space-collapse declaration" do
  def declaration_for(value)
    sheet = Yass::Parser.parse(".x { white-space-collapse: #{value}; }")
    sheet.rules.first.declarations.first
  end

  it "exposes white-space-collapse values" do
    {
      "collapse" => :collapse,
      "preserve" => :preserve,
      "preserve-breaks" => :preserve_breaks,
      "break-spaces" => :break_spaces,
    }.each do |css_value, expected_value|
      declaration = declaration_for(css_value)

      expect(declaration).to be_a(Yass::Declarations::WhiteSpaceCollapse)
      expect(declaration.value).to eq(expected_value)
    end
  end
end
