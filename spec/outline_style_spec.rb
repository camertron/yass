# frozen_string_literal: true

require "spec_helper"

RSpec.describe "outline-style declaration" do
  def declaration_for(value)
    sheet = Yass::Parser.parse(".x { outline-style: #{value}; }")
    sheet.rules.first.declarations.first
  end

  it "exposes solid keyword" do
    declaration = declaration_for("solid")

    expect(declaration).to be_a(Yass::Declarations::OutlineStyle)
    expect(declaration.value).to eq(:solid)
  end

  it "exposes dashed keyword" do
    declaration = declaration_for("dashed")

    expect(declaration).to be_a(Yass::Declarations::OutlineStyle)
    expect(declaration.value).to eq(:dashed)
  end

  it "exposes none keyword" do
    declaration = declaration_for("none")

    expect(declaration).to be_a(Yass::Declarations::OutlineStyle)
    expect(declaration.value).to eq(:none)
  end

  it "exposes auto keyword" do
    declaration = declaration_for("auto")

    expect(declaration).to be_a(Yass::Declarations::OutlineStyle)
    expect(declaration.value).to eq(:auto)
  end
end
