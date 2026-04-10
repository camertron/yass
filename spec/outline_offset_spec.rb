# frozen_string_literal: true

require "spec_helper"

RSpec.describe "outline-offset declaration" do
  def declaration_for(value)
    sheet = Yass::Parser.parse(".x { outline-offset: #{value}; }")
    sheet.rules.first.declarations.first
  end

  it "exposes length values" do
    declaration = declaration_for("2px")

    expect(declaration).to be_a(Yass::Declarations::OutlineOffset)
    expect(declaration.value).to eq("2px")
  end

  it "exposes negative length values" do
    declaration = declaration_for("-0.5em")

    expect(declaration).to be_a(Yass::Declarations::OutlineOffset)
    expect(declaration.value).to eq("-0.5em")
  end
end
