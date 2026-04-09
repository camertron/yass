# frozen_string_literal: true

require "spec_helper"

RSpec.describe "padding declarations" do
  def declaration_for(property, value)
    sheet = Yass::Parser.parse(".x { #{property}: #{value}; }")
    sheet.rules.first.declarations.first
  end

  let(:longhands) do
    [
      ["padding-top",          Yass::Declarations::PaddingTop],
      ["padding-right",        Yass::Declarations::PaddingRight],
      ["padding-bottom",       Yass::Declarations::PaddingBottom],
      ["padding-left",         Yass::Declarations::PaddingLeft],
      ["padding-block-start",  Yass::Declarations::PaddingBlockStart],
      ["padding-block-end",    Yass::Declarations::PaddingBlockEnd],
      ["padding-inline-start", Yass::Declarations::PaddingInlineStart],
      ["padding-inline-end",   Yass::Declarations::PaddingInlineEnd],
    ]
  end

  it "exposes length values" do
    longhands.each do |property, klass|
      declaration = declaration_for(property, "10px")

      expect(declaration).to be_a(klass), "expected #{property}: 10px to parse as #{klass}"
      expect(declaration.value).to be_a(Yass::Declarations::Size::LengthPercentage)

      length = declaration.value.value
      expect(length).to be_a(Yass::Declarations::Length::Absolute)
      expect(length.value).to eq(10.0)
      expect(length.unit).to eq(:px)
    end
  end

  it "exposes percentage values" do
    longhands.each do |property, klass|
      declaration = declaration_for(property, "25%")

      expect(declaration).to be_a(klass), "expected #{property}: 25% to parse as #{klass}"
      expect(declaration.value).to be_a(Yass::Declarations::Size::LengthPercentage)

      percentage = declaration.value.value
      expect(percentage).to be_a(Yass::Declarations::Percentage)
      expect(percentage.value).to be_within(0.00001).of(0.25)
    end
  end

  it "exposes zero length values" do
    longhands.each do |property, klass|
      declaration = declaration_for(property, "0")

      expect(declaration).to be_a(klass), "expected #{property}: 0 to parse as #{klass}"
      expect(declaration.value).to be_a(Yass::Declarations::Size::LengthPercentage)

      length = declaration.value.value
      expect(length).to be_a(Yass::Declarations::Length::Absolute)
      expect(length.value).to eq(0.0)
    end
  end

  it "exposes calc length values" do
    longhands.each do |property, klass|
      declaration = declaration_for(property, "calc(100% - 20px)")

      expect(declaration).to be_a(klass), "expected #{property}: calc(100% - 20px) to parse as #{klass}"
      expect(declaration.value).to be_a(Yass::Declarations::Size::LengthPercentage)
      expect(declaration.value.value).to be_a(Yass::Declarations::Calc)
    end
  end
end
