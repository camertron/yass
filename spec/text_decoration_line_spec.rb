# frozen_string_literal: true

require "spec_helper"

RSpec.describe "text-decoration-line declaration" do
  def declaration_for(value)
    sheet = Yass::Parser.parse(".x { text-decoration-line: #{value}; }")
    sheet.rules.first.declarations.first
  end

  it "exposes none" do
    declaration = declaration_for("none")

    expect(declaration).to be_a(Yass::Declarations::TextDecorationLine)
    expect(declaration.values).to eq([:none])
    expect(declaration.none?).to eq(true)
  end

  it "exposes multiple flags" do
    declaration = declaration_for("underline overline line-through")

    expect(declaration.values).to eq([:underline, :overline, :line_through])
    expect(declaration.none?).to eq(false)
    expect(declaration.underline?).to eq(true)
    expect(declaration.overline?).to eq(true)
    expect(declaration.line_through?).to eq(true)
  end
end
