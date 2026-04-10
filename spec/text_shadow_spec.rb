# frozen_string_literal: true

require "spec_helper"

RSpec.describe "text-shadow declaration" do
  def declaration_for(value)
    sheet = Yass::Parser.parse(".x { text-shadow: #{value}; }")
    sheet.rules.first.declarations.first
  end

  it "exposes text-shadow values" do
    declaration = declaration_for("10px 20px 5px red")

    expect(declaration).to be_a(Yass::Declarations::TextShadow)
    expect(declaration.values).to be_a(Array)
    expect(declaration.values.length).to eq(1)

    shadow = declaration.values.first
    expect(shadow).to be_a(Yass::Declarations::TextShadow::Shadow)

    expect(shadow.horizontal).to be_a(Yass::Declarations::Length::Absolute)
    expect(shadow.horizontal.value).to eq(10.0)
    expect(shadow.horizontal.unit).to eq(:px)

    expect(shadow.vertical).to be_a(Yass::Declarations::Length::Absolute)
    expect(shadow.vertical.value).to eq(20.0)
    expect(shadow.vertical.unit).to eq(:px)

    expect(shadow.blur).to be_a(Yass::Declarations::Length::Absolute)
    expect(shadow.blur.value).to eq(5.0)
    expect(shadow.blur.unit).to eq(:px)

    expect(shadow.color).to be_a(Yass::Declarations::Color::Absolute)
  end

  it "exposes multiple text-shadow values" do
    declaration = declaration_for("2px 2px 5px red, 5px 5px 10px blue")

    expect(declaration.values.length).to eq(2)

    shadow1 = declaration.values[0]
    expect(shadow1.horizontal.value).to eq(2.0)
    expect(shadow1.vertical.value).to eq(2.0)
    expect(shadow1.blur.value).to eq(5.0)

    shadow2 = declaration.values[1]
    expect(shadow2.horizontal.value).to eq(5.0)
    expect(shadow2.vertical.value).to eq(5.0)
    expect(shadow2.blur.value).to eq(10.0)
  end

  it "exposes optional color as nil when not provided" do
    declaration = declaration_for("10px 20px 5px")

    shadow = declaration.values.first
    expect(shadow.color).to be_nil
  end

  it "exposes optional blur as nil when not provided" do
    declaration = declaration_for("10px 20px red")

    shadow = declaration.values.first
    expect(shadow.horizontal.value).to eq(10.0)
    expect(shadow.vertical.value).to eq(20.0)
    expect(shadow.blur).to be_nil
  end
end
