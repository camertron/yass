# frozen_string_literal: true

require "spec_helper"

RSpec.describe "view-transition-class declaration" do
  def declaration_for(value)
    sheet = Yass::Parser.parse(".x { view-transition-class: #{value}; }")
    sheet.rules.first.declarations.first
  end

  it "exposes the none value" do
    declaration = declaration_for("none")

    expect(declaration).to be_a(Yass::Declarations::ViewTransitionClass)
    expect(declaration).to be_none
    expect(declaration.values).to eq([])
  end

  it "exposes a single class name" do
    declaration = declaration_for("foo")

    expect(declaration).to be_a(Yass::Declarations::ViewTransitionClass)
    expect(declaration).not_to be_none
    expect(declaration.values).to eq(["foo"])
  end

  it "exposes multiple class names" do
    declaration = declaration_for("foo bar baz")

    expect(declaration).to be_a(Yass::Declarations::ViewTransitionClass)
    expect(declaration).not_to be_none
    expect(declaration.values).to eq(["foo", "bar", "baz"])
  end
end
