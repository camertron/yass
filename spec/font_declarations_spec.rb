# frozen_string_literal: true

require "spec_helper"

RSpec.describe(Yass) do
  describe "font declarations" do
    def declaration_for(property, value)
      sheet = Yass::Parser.parse(".x { #{property}: #{value}; }")
      sheet.rules.first.declarations.first
    end

    it "exposes font-family values and nested family entries" do
      declaration = declaration_for("font-family", '"Open Sans", serif')

      expect(declaration).to be_a(Yass::Declarations::FontFamily)

      values = declaration.value
      expect(values).to be_a(Yass::Font::Family::Values)

      family_items = values.values
      expect(family_items.size).to eq(2)

      expect(family_items[0]).to be_a(Yass::Font::Family::Name)
      expect(family_items[0].name).to eq("Open Sans")
      expect(family_items[0].syntax).to eq(:quoted)

      expect(family_items[1]).to be_a(Yass::Font::Family::Generic)
      expect(family_items[1].value).to eq(:serif)
    end

    it "exposes font-language-override as normal and tag values" do
      normal_declaration = declaration_for("font-language-override", "normal")

      expect(normal_declaration).to be_a(Yass::Declarations::FontLanguageOverride)
      expect(normal_declaration).to be_normal
      expect(normal_declaration.value).to eq(:normal)

      tag_declaration = declaration_for("font-language-override", '"TRK"')
      expect(tag_declaration).not_to be_normal
      expect(tag_declaration.value).to eq("TRK")
    end

    it "exposes font-optical-sizing values when parser support is enabled" do
      declaration = declaration_for("font-optical-sizing", "none")

      skip("font-optical-sizing is not parsed in this Stylo configuration") unless declaration

      expect(declaration).to be_a(Yass::Declarations::FontOpticalSizing)
      expect(declaration.value).to eq(:none)
    end

    it "exposes font-size keyword and length values" do
      keyword_declaration = declaration_for("font-size", "small")

      expect(keyword_declaration).to be_a(Yass::Declarations::FontSize)
      expect(keyword_declaration.value).to be_a(Yass::Declarations::FontSize::Keyword)
      expect(keyword_declaration.value.keyword).to eq(:small)
      expect(keyword_declaration.value.factor).to eq(1.0)

      length_declaration = declaration_for("font-size", "24px")
      expect(length_declaration.value).to be_a(Yass::Declarations::FontSize::Length)
      expect(length_declaration.value.value).to be_a(Yass::Declarations::Size::LengthPercentage)
    end

    it "exposes font-stretch keyword and percentage variants" do
      keyword_declaration = declaration_for("font-stretch", "condensed")

      expect(keyword_declaration).to be_a(Yass::Declarations::FontStretch)
      expect(keyword_declaration.value).to be_a(Yass::Declarations::FontStretch::Keyword)
      expect(keyword_declaration.value.value).to eq(:condensed)

      percentage_declaration = declaration_for("font-stretch", "87.5%")
      expect(percentage_declaration.value).to be_a(Yass::Declarations::FontStretch::Stretch)
      expect(percentage_declaration.value.value).to be_a(Yass::Declarations::Percentage)
      expect(percentage_declaration.value.value.value).to eq(0.875)
    end

    it "exposes font-style normal and oblique angle variants" do
      normal_declaration = declaration_for("font-style", "normal")

      expect(normal_declaration).to be_a(Yass::Declarations::FontStyle)
      expect(normal_declaration.value).to be_a(Yass::Declarations::FontStyle::Normal)

      oblique_declaration = declaration_for("font-style", "oblique 10deg")
      expect(oblique_declaration.value).to be_a(Yass::Declarations::FontStyle::Oblique)
      expect(oblique_declaration.value.angle).to be_a(Yass::Declarations::Angle)
      expect(oblique_declaration.value.angle.degrees).to eq(10.0)
    end

    it "exposes font-synthesis-weight values" do
      declaration = declaration_for("font-synthesis-weight", "none")

      expect(declaration).to be_a(Yass::Declarations::FontSynthesisWeight)
      expect(declaration.value).to eq(:none)
    end

    it "exposes font-variant-caps values" do
      declaration = declaration_for("font-variant-caps", "small-caps")

      expect(declaration).to be_a(Yass::Declarations::FontVariantCaps)
      expect(declaration.value).to eq(:small_caps)
    end

    it "exposes font-variation-settings entries when parser support is enabled" do
      declaration = declaration_for("font-variation-settings", '"wght" 700, "wdth" 87.5')

      skip("font-variation-settings is not parsed in this Stylo configuration") unless declaration

      expect(declaration).to be_a(Yass::Declarations::FontVariationSettings)

      values = declaration.values
      expect(values.size).to eq(2)

      expect(values[0]).to be_a(Yass::Declarations::FontVariationSettings::Setting)
      expect(values[0].tag).to eq("wght")
      expect(values[0].value).to be_a(Yass::Declarations::Number)
      expect(values[0].value.value).to eq(700.0)

      expect(values[1]).to be_a(Yass::Declarations::FontVariationSettings::Setting)
      expect(values[1].tag).to eq("wdth")
      expect(values[1].value.value).to eq(87.5)
    end

    it "exposes font-weight absolute and bolder variants" do
      absolute_declaration = declaration_for("font-weight", "700")

      expect(absolute_declaration).to be_a(Yass::Declarations::FontWeight)
      expect(absolute_declaration.value).to be_a(Yass::Declarations::FontWeight::Absolute)
      expect(absolute_declaration.value.value).to be_a(Yass::Declarations::Number)
      expect(absolute_declaration.value.value.value).to eq(700.0)

      bolder_declaration = declaration_for("font-weight", "bolder")
      expect(bolder_declaration.value).to be_a(Yass::Declarations::FontWeight::Bolder)
    end
  end
end
