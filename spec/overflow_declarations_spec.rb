# frozen_string_literal: true

require "spec_helper"

RSpec.describe "overflow declarations" do
  def declaration_for(property, value)
    sheet = Yass::Parser.parse(".x { #{property}: #{value}; }")
    sheet.rules.first.declarations.first
  end

  it "exposes overflow-wrap values" do
    {
      "normal" => :normal,
      "break-word" => :break_word,
      "anywhere" => :anywhere,
    }.each do |css_value, expected_value|
      declaration = declaration_for("overflow-wrap", css_value)

      expect(declaration).to be_a(Yass::Declarations::OverflowWrap)
      expect(declaration.value).to eq(expected_value)
    end
  end

  it "exposes overflow directional values" do
    expectations = [
      ["overflow-block", "clip", Yass::Declarations::OverflowBlock, :clip],
      ["overflow-inline", "auto", Yass::Declarations::OverflowInline, :auto],
      ["overflow-x", "hidden", Yass::Declarations::OverflowX, :hidden],
      ["overflow-y", "scroll", Yass::Declarations::OverflowY, :scroll],
    ]

    expectations.each do |property, css_value, klass, expected_value|
      declaration = declaration_for(property, css_value)

      expect(declaration).to be_a(klass)
      expect(declaration.value).to eq(expected_value)
    end
  end

  it "exposes overflow-clip-margin offset and visual-box" do
    declaration = declaration_for("overflow-clip-margin", "border-box 4px")

    expect(declaration).to be_a(Yass::Declarations::OverflowClipMargin)
    expect(declaration.visual_box).to eq(:border_box)

    offset = declaration.offset
    expect(offset).to be_a(Yass::Declarations::Length::Absolute)
    expect(offset.value).to eq(4.0)
    expect(offset.unit).to eq(:px)
  end

  it "defaults overflow-clip-margin visual-box to padding-box" do
    declaration = declaration_for("overflow-clip-margin", "2px")

    expect(declaration).to be_a(Yass::Declarations::OverflowClipMargin)
    expect(declaration.visual_box).to eq(:padding_box)
  end
end
