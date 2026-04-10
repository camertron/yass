# frozen_string_literal: true

require "spec_helper"

RSpec.describe "order declaration" do
  def declaration_for(value)
    sheet = Yass::Parser.parse(".x { order: #{value}; }")
    sheet.rules.first.declarations.first
  end

  it "exposes integer values" do
    declaration = declaration_for("-3")

    expect(declaration).to be_a(Yass::Declarations::Order)
    expect(declaration.value).to eq(-3)
  end
end
