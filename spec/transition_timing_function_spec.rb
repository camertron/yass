# frozen_string_literal: true

require "spec_helper"

RSpec.describe "transition-timing-function declaration" do
  def declaration_for(value)
    sheet = Yass::Parser.parse(".x { transition-timing-function: #{value}; }")
    sheet.rules.first.declarations.first
  end

  it "exposes keyword timing functions" do
    declaration = declaration_for("ease-in")

    expect(declaration).to be_a(Yass::Declarations::TransitionTimingFunction)

    values = declaration.values
    expect(values.size).to eq(1)
    expect(values.first).to be_a(Yass::Declarations::Animation::TimingFunction::Keyword)
    expect(values.first.value).to eq(:ease_in)
  end

  it "exposes cubic-bezier timing functions" do
    declaration = declaration_for("cubic-bezier(0.42, 0, 0.58, 1)")

    value = declaration.values.first
    expect(value).to be_a(Yass::Declarations::Animation::TimingFunction::CubicBezier)
    expect(value.x1.value).to be_within(1e-6).of(0.42)
    expect(value.y1.value).to be_within(1e-6).of(0.0)
    expect(value.x2.value).to be_within(1e-6).of(0.58)
    expect(value.y2.value).to be_within(1e-6).of(1.0)
  end

  it "supports comma-separated transition-timing-function values" do
    declaration = declaration_for("steps(4, jump-start), linear")

    values = declaration.values
    expect(values.size).to eq(2)

    expect(values[0]).to be_a(Yass::Declarations::Animation::TimingFunction::Steps)
    expect(values[0].count).to eq(4)
    expect(values[0].position).to eq(:jump_start)

    expect(values[1]).to be_a(Yass::Declarations::Animation::TimingFunction::Keyword)
    expect(values[1].value).to eq(:linear)
  end
end
