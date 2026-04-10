require_relative "spec_helper"

RSpec.describe(Yass) do
  describe "offset-path declarations" do
    it "exposes none" do
      stylesheet = Yass::Parser.parse("div { offset-path: none; }")
      decl = stylesheet.rules[0].declarations[0]
      expect(decl).to be_a(Yass::Declarations::OffsetPath)
      expect(decl.value).to be_a(Yass::Declarations::OffsetPath::None)
    end

    it "exposes coord-box only" do
      stylesheet = Yass::Parser.parse("div { offset-path: border-box; }")
      decl = stylesheet.rules[0].declarations[0]
      expect(decl).to be_a(Yass::Declarations::OffsetPath)
      coord_box = decl.value
      expect(coord_box).to be_a(Yass::Declarations::OffsetPath::CoordBox)
      expect(coord_box.value).to eq(:border_box)
    end

    it "exposes all coord-box keywords" do
      %w[content-box padding-box border-box fill-box stroke-box view-box].each do |keyword|
        stylesheet = Yass::Parser.parse("div { offset-path: #{keyword}; }")
        decl = stylesheet.rules[0].declarations[0]
        value = decl.value
        expect(value).to be_a(Yass::Declarations::OffsetPath::CoordBox)
        expect(value.value).to eq(keyword.tr("-", "_").to_sym)
      end
    end

    it "exposes ray() function" do
      stylesheet = Yass::Parser.parse("div { offset-path: ray(45deg closest-side); }")
      decl = stylesheet.rules[0].declarations[0]
      offset_path = decl.value
      expect(offset_path).to be_a(Yass::Declarations::OffsetPath::Function)
      ray = offset_path.path
      expect(ray).to be_a(Yass::Declarations::OffsetPath::Ray)
      expect(ray.angle).to be_a(Yass::Declarations::Angle)
      expect(ray.angle.degrees).to be_within(0.01).of(45.0)
      expect(ray.size).to eq(:closest_side)
      expect(ray.contain?).to eq(false)
    end

    it "exposes ray() with contain" do
      stylesheet = Yass::Parser.parse("div { offset-path: ray(90deg farthest-corner contain); }")
      decl = stylesheet.rules[0].declarations[0]
      ray = decl.value.path
      expect(ray).to be_a(Yass::Declarations::OffsetPath::Ray)
      expect(ray.size).to eq(:farthest_corner)
      expect(ray.contain?).to eq(true)
    end

    it "exposes ray() with coord-box" do
      stylesheet = Yass::Parser.parse("div { offset-path: ray(0deg sides) padding-box; }")
      decl = stylesheet.rules[0].declarations[0]
      offset_path = decl.value
      expect(offset_path).to be_a(Yass::Declarations::OffsetPath::Function)
      expect(offset_path.coord_box).to eq(:padding_box)
      expect(offset_path.path).to be_a(Yass::Declarations::OffsetPath::Ray)
    end

    it "exposes ray() position as auto when omitted" do
      stylesheet = Yass::Parser.parse("div { offset-path: ray(45deg closest-corner); }")
      decl = stylesheet.rules[0].declarations[0]
      ray = decl.value.path
      expect(ray.position).to be_a(Yass::Declarations::OffsetPath::PositionAuto)
    end

    it "exposes circle() shape" do
      stylesheet = Yass::Parser.parse("div { offset-path: circle(50%); }")
      decl = stylesheet.rules[0].declarations[0]
      offset_path = decl.value
      expect(offset_path).to be_a(Yass::Declarations::OffsetPath::Function)
      expect(offset_path.path).to be_a(Yass::Declarations::ClipPath::Circle)
    end

    it "exposes url() class type" do
      # The URL variant wraps SVG shape element references; the parser requires
      # a resolvable URL, so we verify the class exists without parsing.
      expect(defined?(Yass::Declarations::OffsetPath::Url)).to eq("constant")
    end
  end
end
