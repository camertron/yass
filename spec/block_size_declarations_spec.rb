# frozen_string_literal: true

require "spec_helper"

RSpec.describe "block size declarations" do
  def declaration_for(property, value)
    sheet = Yass::Parser.parse(".x { #{property}: #{value}; }")
    sheet.rules.first.declarations.first
  end

  describe "max-block-size" do
    it "exposes length values" do
      declaration = declaration_for("max-block-size", "100px")

      expect(declaration).to be_a(Yass::Declarations::MaxBlockSize)
      expect(declaration.size).to be_a(Yass::Declarations::Size::LengthPercentage)

      length = declaration.size.value
      expect(length).to be_a(Yass::Declarations::Length::Absolute)
      expect(length.value).to eq(100.0)
      expect(length.unit).to eq(:px)
    end

    it "exposes percentage values" do
      declaration = declaration_for("max-block-size", "50%")

      expect(declaration).to be_a(Yass::Declarations::MaxBlockSize)
      expect(declaration.size).to be_a(Yass::Declarations::Size::LengthPercentage)

      percentage = declaration.size.value
      expect(percentage).to be_a(Yass::Declarations::Percentage)
      expect(percentage.value).to be_within(0.00001).of(0.5)
    end

    it "exposes the none value" do
      declaration = declaration_for("max-block-size", "none")

      expect(declaration).to be_a(Yass::Declarations::MaxBlockSize)
      expect(declaration.size).to be_a(Yass::Declarations::Size::None)
    end

    it "exposes the max-content keyword" do
      declaration = declaration_for("max-block-size", "max-content")

      expect(declaration).to be_a(Yass::Declarations::MaxBlockSize)
      expect(declaration.size).to be_a(Yass::Declarations::Size::MaxContent)
    end

    it "exposes the min-content keyword" do
      declaration = declaration_for("max-block-size", "min-content")

      expect(declaration).to be_a(Yass::Declarations::MaxBlockSize)
      expect(declaration.size).to be_a(Yass::Declarations::Size::MinContent)
    end
  end

  describe "min-block-size" do
    it "exposes length values" do
      declaration = declaration_for("min-block-size", "20px")

      expect(declaration).to be_a(Yass::Declarations::MinBlockSize)
      expect(declaration.size).to be_a(Yass::Declarations::Size::LengthPercentage)

      length = declaration.size.value
      expect(length).to be_a(Yass::Declarations::Length::Absolute)
      expect(length.value).to eq(20.0)
      expect(length.unit).to eq(:px)
    end

    it "exposes percentage values" do
      declaration = declaration_for("min-block-size", "25%")

      expect(declaration).to be_a(Yass::Declarations::MinBlockSize)
      expect(declaration.size).to be_a(Yass::Declarations::Size::LengthPercentage)

      percentage = declaration.size.value
      expect(percentage).to be_a(Yass::Declarations::Percentage)
      expect(percentage.value).to be_within(0.00001).of(0.25)
    end

    it "exposes the auto value" do
      declaration = declaration_for("min-block-size", "auto")

      expect(declaration).to be_a(Yass::Declarations::MinBlockSize)
      expect(declaration.size).to be_a(Yass::Declarations::Size::Auto)
    end

    it "exposes the min-content keyword" do
      declaration = declaration_for("min-block-size", "min-content")

      expect(declaration).to be_a(Yass::Declarations::MinBlockSize)
      expect(declaration.size).to be_a(Yass::Declarations::Size::MinContent)
    end
  end
end
