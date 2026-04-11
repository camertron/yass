# frozen_string_literal: true

require "spec_helper"

RSpec.describe "zoom declaration" do
  def declaration_for(value)
    sheet = Yass::Parser.parse(".x { zoom: #{value}; }")
    sheet.rules.first.declarations.first
  end

  it "exposes normal" do
    declaration = declaration_for("normal")

    expect(declaration).to be_a(Yass::Declarations::Zoom)
    expect(declaration.value).to be_a(Yass::Declarations::Zoom::Normal)
  end

  it "exposes numeric values" do
    declaration = declaration_for("2")

    expect(declaration).to be_a(Yass::Declarations::Zoom)
    expect(declaration.value).to be_a(Yass::Declarations::Zoom::Value)
    expect(declaration.value.value).to be_a(Yass::Declarations::Number)
    expect(declaration.value.value.value).to be_within(0.001).of(2.0)
  end

  it "exposes percentage values" do
    declaration = declaration_for("150%")

    expect(declaration).to be_a(Yass::Declarations::Zoom)
    expect(declaration.value).to be_a(Yass::Declarations::Zoom::Value)
    expect(declaration.value.value).to be_a(Yass::Declarations::Percentage)
    expect(declaration.value.value.value).to be_within(0.001).of(1.5)
  end
end
