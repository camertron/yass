# frozen_string_literal: true

require "spec_helper"

RSpec.describe "opacity declaration" do
  def declaration_for(value)
    sheet = Yass::Parser.parse(".x { opacity: #{value}; }")
    sheet.rules.first.declarations.first
  end

  it "exposes numeric opacity values" do
    declaration = declaration_for("0.4")
    expect(declaration).to be_a(Yass::Declarations::Opacity)
    expect(declaration.value).to be_a(Yass::Declarations::Number)
    expect(declaration.value.value).to be_within(0.001).of(0.4)
  end

  it "converts percentages to numbers" do
    declaration = declaration_for("75%")
    expect(declaration).to be_a(Yass::Declarations::Opacity)
    expect(declaration.value).to be_a(Yass::Declarations::Number)
    expect(declaration.value.value).to be_within(0.001).of(0.75)
  end
end
