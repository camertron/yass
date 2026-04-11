# frozen_string_literal: true

require "spec_helper"

RSpec.describe "z-index declaration" do
  def declaration_for(value)
    sheet = Yass::Parser.parse(".x { z-index: #{value}; }")
    sheet.rules.first.declarations.first
  end

  it "wraps integer values" do
    declaration = declaration_for("42")

    expect(declaration).to be_a(Yass::Declarations::ZIndex)
    expect(declaration.value).to be_a(Yass::Declarations::ZIndex::Integer)
    expect(declaration.value.value).to eq(42)
  end

  it "wraps negative integer values" do
    declaration = declaration_for("-5")

    expect(declaration).to be_a(Yass::Declarations::ZIndex)
    expect(declaration.value).to be_a(Yass::Declarations::ZIndex::Integer)
    expect(declaration.value.value).to eq(-5)
  end

  it "wraps the auto keyword" do
    declaration = declaration_for("auto")

    expect(declaration).to be_a(Yass::Declarations::ZIndex)
    expect(declaration.value).to be_a(Yass::Declarations::ZIndex::Auto)
  end
end
