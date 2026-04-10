# frozen_string_literal: true

require "spec_helper"

RSpec.describe "text-overflow declaration" do
  def declaration_for(value)
    sheet = Yass::Parser.parse(".x { text-overflow: #{value}; }")
    sheet.rules.first.declarations.first
  end

  it "exposes one-sided values as logical sides" do
    declaration = declaration_for("ellipsis")

    expect(declaration).to be_a(Yass::Declarations::TextOverflow)
    expect(declaration.sides_are_logical?).to eq(true)
    expect(declaration.first).to be_a(Yass::Declarations::TextOverflow::Clip)
    expect(declaration.second).to be_a(Yass::Declarations::TextOverflow::Ellipsis)
  end

  it "exposes two-sided values as physical sides" do
    declaration = declaration_for("clip ellipsis")

    expect(declaration.sides_are_logical?).to eq(false)
    expect(declaration.first).to be_a(Yass::Declarations::TextOverflow::Clip)
    expect(declaration.second).to be_a(Yass::Declarations::TextOverflow::Ellipsis)
  end

  it "exposes string-side values" do
    declaration = declaration_for('".."')
    second = declaration.second

    expect(second).to be_a(Yass::Declarations::TextOverflow::String)
    expect(second.value).to eq("..")
  end
end
