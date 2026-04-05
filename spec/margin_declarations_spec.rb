# frozen_string_literal: true

require "spec_helper"

RSpec.describe "margin declarations" do
  def declaration_for(property, value)
    sheet = Yass::Parser.parse(".x { #{property}: #{value}; }")
    sheet.rules.first.declarations.first
  end

  LONGHANDS = [
    ["margin-top",          Yass::Declarations::MarginTop],
    ["margin-right",        Yass::Declarations::MarginRight],
    ["margin-bottom",       Yass::Declarations::MarginBottom],
    ["margin-left",         Yass::Declarations::MarginLeft],
    ["margin-block-start",  Yass::Declarations::MarginBlockStart],
    ["margin-block-end",    Yass::Declarations::MarginBlockEnd],
    ["margin-inline-start", Yass::Declarations::MarginInlineStart],
    ["margin-inline-end",   Yass::Declarations::MarginInlineEnd],
  ].freeze

  it "exposes length values" do
    LONGHANDS.each do |property, klass|
      declaration = declaration_for(property, "10px")

      expect(declaration).to be_a(klass), "expected #{property}: 10px to parse as #{klass}"
      expect(declaration.value).to be_a(Yass::Declarations::Margin::LengthPercentage)

      length = declaration.value.value
      expect(length).to be_a(Yass::Declarations::Length::Absolute)
      expect(length.value).to eq(10.0)
      expect(length.unit).to eq(:px)
    end
  end

  it "exposes percentage values" do
    LONGHANDS.each do |property, klass|
      declaration = declaration_for(property, "25%")

      expect(declaration).to be_a(klass), "expected #{property}: 25% to parse as #{klass}"
      expect(declaration.value).to be_a(Yass::Declarations::Margin::LengthPercentage)

      percentage = declaration.value.value
      expect(percentage).to be_a(Yass::Declarations::Percentage)
      expect(percentage.value).to be_within(0.00001).of(0.25)
    end
  end

  it "exposes auto values" do
    LONGHANDS.each do |property, klass|
      declaration = declaration_for(property, "auto")

      expect(declaration).to be_a(klass), "expected #{property}: auto to parse as #{klass}"
      expect(declaration.value).to be_a(Yass::Declarations::Margin::Auto)
    end
  end

  it "exposes zero length values" do
    LONGHANDS.each do |property, klass|
      declaration = declaration_for(property, "0")

      expect(declaration).to be_a(klass), "expected #{property}: 0 to parse as #{klass}"
      expect(declaration.value).to be_a(Yass::Declarations::Margin::LengthPercentage)

      length = declaration.value.value
      expect(length).to be_a(Yass::Declarations::Length::Absolute)
      expect(length.value).to eq(0.0)
    end
  end

  it "exposes calc length values" do
    LONGHANDS.each do |property, klass|
      declaration = declaration_for(property, "calc(100% - 20px)")

      expect(declaration).to be_a(klass), "expected #{property}: calc(100% - 20px) to parse as #{klass}"
      expect(declaration.value).to be_a(Yass::Declarations::Margin::LengthPercentage)
    end
  end
end
