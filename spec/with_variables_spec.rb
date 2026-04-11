# frozen_string_literal: true

require "spec_helper"

RSpec.describe "with-variables declaration" do
  def declaration_for(value)
    sheet = Yass::Parser.parse(".x { color: #{value}; }")
    sheet.rules.first.declarations.first
  end

  it "exposes declarations containing var() as WithVariables" do
    declaration = declaration_for("var(--theme)")

    expect(declaration).to be_a(Yass::Declarations::WithVariables)
    expect(declaration.value).to eq("var(--theme)")
  end
end
