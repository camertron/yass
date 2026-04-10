# frozen_string_literal: true

require "spec_helper"

RSpec.describe "outline-width declaration" do
  def declaration_for(value)
    sheet = Yass::Parser.parse(".x { outline-width: #{value}; }")
    sheet.rules.first.declarations.first
  end

  it "exposes keyword values as symbols" do
    declaration = declaration_for("medium")

    expect(declaration).to be_a(Yass::Declarations::OutlineWidth)
    expect(declaration.value).to eq(:medium)
  end

  it "exposes length values as strings" do
    declaration = declaration_for("2px")

    expect(declaration).to be_a(Yass::Declarations::OutlineWidth)
    expect(declaration.value).to eq("2px")
  end
end
