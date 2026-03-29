# frozen_string_literal: true

RSpec.describe(Yass) do
  describe "color declarations" do
    def color_for(css)
      sheet = Yass::Parser.parse(".x { background-color: #{css}; }")
      sheet.rules.first.declarations.first.color
    end

    it "exposes background-color as a BackgroundColor declaration" do
      sheet = Yass::Parser.parse(".x { background-color: red; }")
      declaration = sheet.rules.first.declarations.first
      expect(declaration).to be_a(Yass::Declarations::BackgroundColor)
    end

    it "parses currentColor" do
      color = color_for("currentColor")

      expect(color).to be_a(Yass::Declarations::Color::CurrentColor)
      expect(color).not_to be_a(Yass::Declarations::Color::Absolute)
      expect(color).not_to be_a(Yass::Declarations::Color::ColorFunction)
      expect(color).not_to be_a(Yass::Declarations::Color::SystemColor)
    end

    it "parses a named color as absolute" do
      color = color_for("red")
      expect(color).to be_a(Yass::Declarations::Color::Absolute)
      expect(color.authored).to eq("red")

      abs_color = color.color
      expect(abs_color).to be_a(Yass::Declarations::Color::AbsoluteColor)
      expect(abs_color.color_space).to eq(:srgb)
    end

    it "parses a hex color as absolute" do
      color = color_for("#ff0000")
      expect(color).to be_a(Yass::Declarations::Color::Absolute)
      abs_color = color.color
      expect(abs_color.color_space).to eq(:srgb)
      expect(abs_color.alpha).to_not be_nil
    end

    it "parses transparent as an absolute color" do
      color = color_for("transparent")
      expect(color).to be_a(Yass::Declarations::Color::Absolute)
      expect(color.color.transparent?).to eq(true)
    end

    it "exposes absolute color components and flags for a named color" do
      color = color_for("red")
      expect(color).to be_a(Yass::Declarations::Color::Absolute)

      absolute_color = color.color
      expect(absolute_color).to be_a(Yass::Declarations::Color::AbsoluteColor)
      expect(absolute_color.color_space).to eq(:srgb)
      expect(absolute_color.alpha).to eq(1.0)
      expect(absolute_color).to be_legacy_syntax
      expect(absolute_color).not_to be_transparent

      components = absolute_color.components
      expect(components).to be_a(Yass::Declarations::Color::ColorComponents)
      expect([components.c0, components.c1, components.c2]).to eq([1.0, 0.0, 0.0])
    end

    it "exposes transparent alpha and flags" do
      color = color_for("transparent")
      expect(color).to be_a(Yass::Declarations::Color::Absolute)

      absolute_color = color.color
      expect(absolute_color.alpha).to eq(0.0)
      expect(absolute_color).to be_transparent
    end

    it "parses relative rgb() as a color function with origin and channel keywords" do
      color = color_for("rgb(from red r g b / alpha)")
      expect(color).to be_a(Yass::Declarations::Color::ColorFunction)

      expect(color.kind).to eq(:rgb)
      expect(color).to be_has_origin_color
      expect(color.origin_color).to be_a(Yass::Declarations::Color::Absolute)

      components = color.components
      expect(components.size).to eq(3)
      expect(components.map(&:kind)).to eq([:channel_keyword, :channel_keyword, :channel_keyword])
      expect(components.map { |component| component.channel_keyword.value }).to eq([:r, :g, :b])

      alpha = color.alpha
      expect(alpha.kind).to eq(:channel_keyword)
      expect(alpha.channel_keyword.value).to eq(:alpha)
    end

    it "parses color-mix items with normalized percentages" do
      color = color_for("color-mix(in srgb, red, blue)")
      expect(color).to be_a(Yass::Declarations::Color::ColorMix)

      expect(color).to be_normalize_weights
      expect(color).to be_result_in_modern_syntax

      interpolation = color.interpolation
      expect(interpolation).to be_a(Yass::Declarations::Color::ColorInterpolationMethod)
      expect(interpolation.space).to eq(:srgb)
      expect(interpolation.hue).to eq(:shorter)

      items = color.items
      expect(items.size).to eq(2)
      expect(items[0]).to be_a(Yass::Declarations::Color::ColorMixItem)
      expect(items[1]).to be_a(Yass::Declarations::Color::ColorMixItem)
      expect(items.map { |item| item.color.class }).to eq([
        Yass::Declarations::Color::Absolute,
        Yass::Declarations::Color::Absolute,
      ])
      expect(items.map { |item| item.percentage.value }).to eq([0.5, 0.5])
    end

    it "parses a system color keyword as a system color" do
      color = color_for("CanvasText")
      expect(color).to be_a(Yass::Declarations::Color::SystemColor)
      expect(color.value).to eq(:canvas_text)
    end

    it "parses rgb() as an absolute color" do
      color = color_for("rgb(255 0 128)")
      expect(color).to be_a(Yass::Declarations::Color::Absolute)

      absolute_color = color.color
      expect(absolute_color.color_space).to eq(:srgb)
      expect(absolute_color.alpha).to eq(1.0)

      components = absolute_color.components
      expect(components).to be_a(Yass::Declarations::Color::ColorComponents)
      expect(components.c0).to eq(1.0)
      expect(components.c1).to eq(0.0)
      expect(components.c2).to be_within(0.000001).of(0.501960813999176)
    end

    it "parses hsl() as an absolute color in hsl space" do
      color = color_for("hsl(120deg 100% 50%)")
      expect(color).to be_a(Yass::Declarations::Color::Absolute)
      expect(color.color.color_space).to eq(:hsl)
    end

    it "parses lch() as an absolute color in lch space" do
      color = color_for("lch(50% 30 120deg)")
      expect(color).to be_a(Yass::Declarations::Color::Absolute)
      expect(color.color.color_space).to eq(:lch)
    end

    it "parses color() as an absolute color in a named color space" do
      color = color_for("color(display-p3 1 0 0)")
      expect(color).to be_a(Yass::Declarations::Color::Absolute)
      expect(color.color.color_space).to eq(:display_p3)
    end

    it "parses color-mix()" do
      color = color_for("color-mix(in srgb, red, blue)")
      expect(color).to be_a(Yass::Declarations::Color::ColorMix)

      interp = color.interpolation
      expect(interp).to be_a(Yass::Declarations::Color::ColorInterpolationMethod)
      expect(interp.space).to eq(:srgb)

      items = color.items
      expect(items.size).to eq(2)
      expect(items[0].color).to be_a(Yass::Declarations::Color::Absolute)
      expect(items[1].color).to be_a(Yass::Declarations::Color::Absolute)
    end

    it "parses light-dark()" do
      color = color_for("light-dark(white, black)")
      expect(color).to be_a(Yass::Declarations::Color::LightDark)
      expect(color.light).to be_a(Yass::Declarations::Color::Absolute)
      expect(color.dark).to be_a(Yass::Declarations::Color::Absolute)
      expect(color.light.authored).to eq("white")
      expect(color.dark.authored).to eq("black")
    end
  end
end
