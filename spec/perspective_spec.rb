# frozen_string_literal: true

require "spec_helper"

RSpec.describe "perspective declaration" do
  def declaration_for(value)
    sheet = Yass::Parser.parse(".x { perspective: #{value}; }")
    sheet.rules.first.declarations.first
  end

  it "exposes the none keyword" do
    declaration = declaration_for("none")
    expect(declaration).to be_a(Yass::Declarations::Perspective)
    expect(declaration.value).to be_a(Yass::Declarations::Perspective::None)
  end

  it "exposes a length value" do
    declaration = declaration_for("20px")
    expect(declaration).to be_a(Yass::Declarations::Perspective)
    expect(declaration.value).to be_a(Yass::Declarations::Perspective::Length)
    expect(declaration.value.value).to be_a(Yass::Declarations::Length::Absolute)
  end
end
