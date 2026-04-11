# frozen_string_literal: true

require "spec_helper"

RSpec.describe "transition-property declaration" do
  def declaration_for(value)
    sheet = Yass::Parser.parse(".x { transition-property: #{value}; }")
    sheet.rules.first.declarations.first
  end

  it "exposes non-custom transition properties" do
    declaration = declaration_for("opacity")

    expect(declaration).to be_a(Yass::Declarations::TransitionProperty)

    values = declaration.values
    expect(values.size).to eq(1)
    expect(values.first).to be_a(Yass::Declarations::TransitionProperty::NonCustom)
    expect(values.first.name).to eq("opacity")
    expect(values.first).not_to be_all
  end

  it "marks all as a non-custom all value" do
    declaration = declaration_for("all")

    value = declaration.values.first
    expect(value).to be_a(Yass::Declarations::TransitionProperty::NonCustom)
    expect(value.name).to eq("all")
    expect(value).to be_all
  end

  it "exposes custom properties" do
    declaration = declaration_for("--my-transition")

    value = declaration.values.first
    expect(value).to be_a(Yass::Declarations::TransitionProperty::Custom)
    expect(value.name).to eq("--my-transition")
  end

  it "exposes none as an unsupported value" do
    declaration = declaration_for("none")

    value = declaration.values.first
    expect(value).to be_a(Yass::Declarations::TransitionProperty::Unsupported)
    expect(value.name).to eq("none")
    expect(value).to be_none
  end

  it "supports comma-separated transition-property values" do
    declaration = declaration_for("opacity, --x, width")

    values = declaration.values
    expect(values.size).to eq(3)

    expect(values[0]).to be_a(Yass::Declarations::TransitionProperty::NonCustom)
    expect(values[0].name).to eq("opacity")

    expect(values[1]).to be_a(Yass::Declarations::TransitionProperty::Custom)
    expect(values[1].name).to eq("--x")

    expect(values[2]).to be_a(Yass::Declarations::TransitionProperty::NonCustom)
    expect(values[2].name).to eq("width")
    expect(values[2]).not_to be_all
  end
end
