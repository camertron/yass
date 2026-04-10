# frozen_string_literal: true

require "spec_helper"

RSpec.describe "text-indent declaration" do
  def declaration_for(value)
    sheet = Yass::Parser.parse(".x { text-indent: #{value}; }")
    sheet.rules.first.declarations.first
  end

  it "exposes the indent length and default flags" do
    declaration = declaration_for("2px")

    expect(declaration).to be_a(Yass::Declarations::TextIndent)
    length = declaration.length

    expect(length).to be_a(Yass::Declarations::Length::Absolute)
    expect(length.value).to eq(2.0)
    expect(length.unit).to eq(:px)
    expect(declaration).not_to be_hanging
    expect(declaration).not_to be_each_line
  end

  it "exposes calc lengths" do
    declaration = declaration_for("calc(1em + 2px)")

    expect(declaration).to be_a(Yass::Declarations::TextIndent)
    expect(declaration.length).to be_a(Yass::Declarations::Calc)
    expect(declaration).not_to be_hanging
    expect(declaration).not_to be_each_line
  end
end
