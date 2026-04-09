# frozen_string_literal: true

require "spec_helper"

RSpec.describe "list-style-type declaration" do
  def declaration_for(value)
    sheet = Yass::Parser.parse(".x { list-style-type: #{value}; }")
    sheet.rules.first.declarations.first
  end

  it "exposes named, none, string, and symbols values" do
    named = declaration_for("disc")

    expect(named).to be_a(Yass::Declarations::ListStyleType)
    expect(named.value).to be_a(Yass::Declarations::ListStyleType::Name)
    expect(named.value.value).to eq(:disc)
    expect(named).to be_bullet

    none = declaration_for("none")

    expect(none.value).to be_a(Yass::Declarations::ListStyleType::None)
    expect(none).not_to be_bullet

    string = declaration_for('"marker"')

    expect(string.value).to be_a(Yass::Declarations::ListStyleType::String)
    expect(string.value.value).to eq("marker")

    symbols = declaration_for('symbols(cyclic "*" "-")')

    expect(symbols.value).to be_a(Yass::Declarations::ListStyleType::Symbols)
    expect(symbols.value.type).to eq(:cyclic)
    expect(symbols.value.symbols).to eq(["*", "-"])
  end
end
