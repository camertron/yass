# frozen_string_literal: true

require "spec_helper"

RSpec.describe "word-break declaration" do
  def declaration_for(value)
    sheet = Yass::Parser.parse(".x { word-break: #{value}; }")
    sheet.rules.first.declarations.first
  end

  it "parses word-break: normal" do
    decl = declaration_for("normal")
    expect(decl).to be_a(Yass::Declarations::WordBreak)
    expect(decl.value).to eq(:normal)
  end

  it "parses word-break: break-all" do
    decl = declaration_for("break-all")
    expect(decl).to be_a(Yass::Declarations::WordBreak)
    expect(decl.value).to eq(:break_all)
  end

  it "parses word-break: keep-all" do
    decl = declaration_for("keep-all")
    expect(decl).to be_a(Yass::Declarations::WordBreak)
    expect(decl.value).to eq(:keep_all)
  end
end
