# frozen_string_literal: true

require "spec_helper"

RSpec.describe "alignment declarations" do
  def declaration_for(property, value)
    sheet = Yass::Parser.parse(".x { #{property}: #{value}; }")
    sheet.rules.first.declarations.first
  end

  it "exposes justify-content values" do
    {
      "normal" => :normal,
      "space-between" => :space_between,
      "center" => :center,
    }.each do |css_value, expected_value|
      declaration = declaration_for("justify-content", css_value)

      expect(declaration).to be_a(Yass::Declarations::JustifyContent)
      expect(declaration.value).to eq(expected_value)
    end
  end

  it "exposes justify-items values" do
    {
      "normal" => :normal,
      "left" => :left,
      "self-end" => :self_end,
    }.each do |css_value, expected_value|
      declaration = declaration_for("justify-items", css_value)

      expect(declaration).to be_a(Yass::Declarations::JustifyItems)
      expect(declaration.value).to eq(expected_value)
    end
  end

  it "exposes justify-self values" do
    {
      "auto" => :auto,
      "end" => :end,
      "self-start" => :self_start,
    }.each do |css_value, expected_value|
      declaration = declaration_for("justify-self", css_value)

      expect(declaration).to be_a(Yass::Declarations::JustifySelf)
      expect(declaration.value).to eq(expected_value)
    end
  end
end
