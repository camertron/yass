# frozen_string_literal: true

require "spec_helper"

RSpec.describe "line-height declaration" do
  def declaration_for(value)
    sheet = Yass::Parser.parse(".x { line-height: #{value}; }")
    sheet.rules.first.declarations.first
  end

  it "exposes the normal keyword" do
    declaration = declaration_for("normal")
    expect(declaration).to be_a(Yass::Declarations::LineHeight)
    expect(declaration.value).to be_a(Yass::Declarations::LineHeight::Normal)
  end

  it "exposes a unitless number" do
    declaration = declaration_for("1.5")
    expect(declaration).to be_a(Yass::Declarations::LineHeight)
    expect(declaration.value).to be_a(Yass::Declarations::Number)
    expect(declaration.value.value).to be_within(0.001).of(1.5)
  end

  it "exposes a length value" do
    declaration = declaration_for("20px")
    expect(declaration).to be_a(Yass::Declarations::LineHeight)
    expect(declaration.value).to be_a(Yass::Declarations::Length::Absolute)
  end

  it "exposes a percentage value" do
    declaration = declaration_for("150%")
    expect(declaration).to be_a(Yass::Declarations::LineHeight)
    expect(declaration.value).to be_a(Yass::Declarations::Percentage)
  end
end
