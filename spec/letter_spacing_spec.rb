# frozen_string_literal: true

require "spec_helper"

RSpec.describe "letter-spacing declaration" do
  def declaration_for(value)
    sheet = Yass::Parser.parse(".x { letter-spacing: #{value}; }")
    sheet.rules.first.declarations.first
  end

  it "exposes normal" do
    declaration = declaration_for("normal")

    expect(declaration).to be_a(Yass::Declarations::LetterSpacing)
    expect(declaration.value).to be_a(Yass::Declarations::LetterSpacing::Normal)
  end

  it "exposes length values" do
    declaration = declaration_for("2px")

    expect(declaration).to be_a(Yass::Declarations::LetterSpacing)
    length = declaration.value
    expect(length).to be_a(Yass::Declarations::Length::Absolute)
    expect(length.value).to eq(2.0)
    expect(length.unit).to eq(:px)
  end

  it "exposes calc values" do
    declaration = declaration_for("calc(1em + 2px)")

    expect(declaration).to be_a(Yass::Declarations::LetterSpacing)
    expect(declaration.value).to be_a(Yass::Declarations::Calc)
  end
end
