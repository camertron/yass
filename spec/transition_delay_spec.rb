# frozen_string_literal: true

require "spec_helper"

RSpec.describe "transition-delay declaration" do
  def declaration_for(value)
    sheet = Yass::Parser.parse(".x { transition-delay: #{value}; }")
    sheet.rules.first.declarations.first
  end

  it "exposes transition-delay values" do
    declaration = declaration_for("150ms")

    expect(declaration).to be_a(Yass::Declarations::TransitionDelay)

    values = declaration.values
    expect(values.size).to eq(1)
    expect(values.first).to be_a(Yass::Declarations::Time)
    expect(values.first.seconds).to be_within(1e-6).of(0.15)
    expect(values.first.unit).to eq("ms")
  end

  it "supports comma-separated transition-delay values" do
    declaration = declaration_for("120ms, 2s")

    expect(declaration).to be_a(Yass::Declarations::TransitionDelay)

    values = declaration.values
    expect(values.size).to eq(2)

    expect(values[0]).to be_a(Yass::Declarations::Time)
    expect(values[0].seconds).to be_within(1e-6).of(0.12)
    expect(values[0].unit).to eq("ms")

    expect(values[1]).to be_a(Yass::Declarations::Time)
    expect(values[1].seconds).to be_within(1e-6).of(2.0)
    expect(values[1].unit).to eq("s")
  end
end
