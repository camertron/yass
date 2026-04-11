# frozen_string_literal: true

require "spec_helper"

RSpec.describe "will-change declaration" do
  def declaration_for(value)
    sheet = Yass::Parser.parse(".x { will-change: #{value}; }")
    sheet.rules.first.declarations.first
  end

  it "exposes auto as an empty feature list" do
    declaration = declaration_for("auto")

    expect(declaration).to be_a(Yass::Declarations::WillChange)
    expect(declaration).to be_auto
    expect(declaration.values).to eq([])

    expect(declaration).not_to be_transform
    expect(declaration).not_to be_opacity
    expect(declaration).not_to be_scroll
  end

  it "exposes feature names and change bits for animatable properties" do
    declaration = declaration_for("transform, opacity")

    expect(declaration).to be_a(Yass::Declarations::WillChange)
    expect(declaration).not_to be_auto
    expect(declaration.values).to eq(["transform", "opacity"])

    expect(declaration).to be_transform
    expect(declaration).to be_opacity
    expect(declaration).to be_backdrop_root
  end

  it "supports non-property features and property-derived change bits" do
    declaration = declaration_for("scroll-position, filter")

    expect(declaration.values).to eq(["scroll-position", "filter"])
    expect(declaration).to be_scroll
    expect(declaration).to be_fixpos_cb_non_svg
  end

  it "preserves custom identifiers even when they do not map to change bits" do
    declaration = declaration_for("my-custom-feature")

    expect(declaration.values).to eq(["my-custom-feature"])
    expect(declaration).not_to be_auto
    expect(declaration).not_to be_transform
    expect(declaration).not_to be_opacity
    expect(declaration).not_to be_scroll
  end
end
