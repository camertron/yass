# frozen_string_literal: true

require "spec_helper"

RSpec.describe(Yass) do
  it "parses a basic stylesheet" do
    sheet = Yass::Parser.parse(<<~CSS)
      h1, .foo {
        color: red;
      }
    CSS

    expect(sheet.rules.size).to eq(1)

    selectors = sheet.rules.first.selectors
    expect(selectors.size).to eq(2)

    first_selector_children = selectors.first.children
    expect(first_selector_children.size).to eq(1)
    expect(first_selector_children[0].kind).to eq(:local_name)

    second_selector_children = selectors.last.children
    expect(second_selector_children.size).to eq(1)
    expect(second_selector_children[0].kind).to eq(:klass)
  end

  it "exposes backface visibility declarations" do
    sheet = Yass::Parser.parse(<<~CSS)
      .card {
        backface-visibility: hidden;
      }
    CSS

    declaration = sheet.rules.first.declarations.first

    expect(declaration).to be_a(Yass::Declarations::BackfaceVisibility)
    expect(declaration.value).to eq(:hidden)
  end

  it "exposes background attachment declarations" do
    sheet = Yass::Parser.parse(<<~CSS)
      .hero {
        background-attachment: fixed, scroll;
      }
    CSS

    declaration = sheet.rules.first.declarations.first

    expect(declaration).to be_a(Yass::Declarations::BackgroundAttachment)
    expect(declaration.values).to eq([:fixed, :scroll])
  end

  it "exposes background clip declarations" do
    sheet = Yass::Parser.parse(<<~CSS)
      .hero {
        background-clip: padding-box, content-box;
      }
    CSS

    declaration = sheet.rules.first.declarations.first

    expect(declaration).to be_a(Yass::Declarations::BackgroundClip)
    expect(declaration.values).to eq([:"padding-box", :"content-box"])
  end

  it "exposes background image declarations" do
    sheet = Yass::Parser.parse(<<~CSS)
      .hero {
        background-image: none, linear-gradient(red, blue);
      }
    CSS

    declaration = sheet.rules.first.declarations.first

    expect(declaration).to be_a(Yass::Declarations::BackgroundImage)
    values = declaration.values

    expect(values.size).to eq(2)
    expect(values[0]).to be_a(Yass::Declarations::Image::None)

    gradient = values[1]
    expect(gradient).to be_a(Yass::Declarations::Image::LinearGradient)
    expect(gradient.repeating?).to eq(false)

    direction = gradient.direction
    expect(direction).to be_a(Yass::Declarations::Image::VerticalLineDirection)
    expect(direction.direction).to eq(:bottom)

    items = gradient.items
    expect(items.size).to eq(2)
    expect(items[0]).to be_a(Yass::Declarations::Image::Gradient::SimpleColorStopLength)
    expect(items[0].color).to be_a(Yass::Declarations::Color::Absolute)
  end

  it "exposes background repeat declarations" do
    sheet = Yass::Parser.parse(<<~CSS)
      .hero {
        background-repeat: repeat-x, space round, no-repeat;
      }
    CSS

    declaration = sheet.rules.first.declarations.first

    expect(declaration).to be_a(Yass::Declarations::BackgroundRepeat)

    values = declaration.values
    expect(values.size).to eq(3)

    expect(values[0]).to be_a(Yass::Declarations::BackgroundRepeatValue)
    expect(values[0].horizontal).to eq(:repeat)
    expect(values[0].vertical).to eq(:no_repeat)

    expect(values[1].horizontal).to eq(:space)
    expect(values[1].vertical).to eq(:round)

    expect(values[2].horizontal).to eq(:no_repeat)
    expect(values[2].vertical).to eq(:no_repeat)
  end

  it "exposes background size declarations" do
    sheet = Yass::Parser.parse(<<~CSS)
      .hero {
        background-size: cover, contain, 25% auto;
      }
    CSS

    declaration = sheet.rules.first.declarations.first

    expect(declaration).to be_a(Yass::Declarations::BackgroundSize)

    values = declaration.values
    expect(values.size).to eq(3)

    expect(values[0]).to be_a(Yass::Declarations::BackgroundSize::Cover)
    expect(values[1]).to be_a(Yass::Declarations::BackgroundSize::Contain)

    explicit_size = values[2]
    expect(explicit_size).to be_a(Yass::Declarations::BackgroundSize::ExplicitSize)
    expect(explicit_size.width).to be_a(Yass::Declarations::Percentage)
    expect(explicit_size.height).to be_a(Yass::Declarations::BackgroundSize::Auto)
  end

  it "exposes baseline shift declarations" do
    sheet = Yass::Parser.parse(<<~CSS)
      .a {
        baseline-shift: super;
      }

      .b {
        baseline-shift: 25%;
      }
    CSS

    keyword_declaration = sheet.rules[0].declarations.first

    expect(keyword_declaration).to be_a(Yass::Declarations::BaselineShift::Keyword)
    expect(keyword_declaration.value).to eq(:super)

    length_declaration = sheet.rules[1].declarations.first

    expect(length_declaration).to be_a(Yass::Declarations::BaselineShift::Length)

    length = length_declaration.value
    expect(length).to be_a(Yass::Declarations::Percentage)
    expect(length.value).to eq(0.25)
  end

  describe "border style declarations" do
    def border_style_declaration(property, value)
      sheet = Yass::Parser.parse(".x { #{property}: #{value}; }")
      sheet.rules.first.declarations.first
    end

    it "exposes side and logical border styles as declaration wrappers" do
      expectations = [
        ["border-top-style", :dotted, Yass::Declarations::BorderTopStyle],
        ["border-right-style", :dashed, Yass::Declarations::BorderRightStyle],
        ["border-bottom-style", :double, Yass::Declarations::BorderBottomStyle],
        ["border-left-style", :solid, Yass::Declarations::BorderLeftStyle],
        ["border-inline-start-style", :groove, Yass::Declarations::BorderInlineStartStyle],
        ["border-inline-end-style", :ridge, Yass::Declarations::BorderInlineEndStyle],
        ["border-block-start-style", :inset, Yass::Declarations::BorderBlockStartStyle],
        ["border-block-end-style", :outset, Yass::Declarations::BorderBlockEndStyle],
      ]

      expectations.each do |property, value, klass|
        declaration = border_style_declaration(property, value)

        expect(declaration).to be_a(klass)
        expect(declaration.value).to eq(value)
      end
    end
  end

  describe "border collapse declarations" do
    def border_collapse_declaration(value)
      sheet = Yass::Parser.parse(".x { border-collapse: #{value}; }")
      sheet.rules.first.declarations.first
    end

    it "exposes border collapse values as declaration wrappers" do
      {
        "separate" => :separate,
        "collapse" => :collapse,
      }.each do |css_value, expected_value|
        declaration = border_collapse_declaration(css_value)

        expect(declaration).to be_a(Yass::Declarations::BorderCollapse)
        expect(declaration.value).to eq(expected_value)
      end
    end
  end

  describe "border spacing declarations" do
    def border_spacing_declaration(value)
      sheet = Yass::Parser.parse(".x { border-spacing: #{value}; }")
      sheet.rules.first.declarations.first
    end

    it "exposes horizontal and vertical border spacing lengths" do
      declaration = border_spacing_declaration("4px 6px")

      expect(declaration).to be_a(Yass::Declarations::BorderSpacing)

      expect(declaration.horizontal).to be_a(Yass::Declarations::Length::Absolute)
      expect(declaration.horizontal.value).to eq(4.0)
      expect(declaration.horizontal.unit).to eq(:px)

      expect(declaration.vertical).to be_a(Yass::Declarations::Length::Absolute)
      expect(declaration.vertical.value).to eq(6.0)
      expect(declaration.vertical.unit).to eq(:px)
    end
  end

  describe "box sizing declarations" do
    def box_sizing_declaration(value)
      sheet = Yass::Parser.parse(".x { box-sizing: #{value}; }")
      sheet.rules.first.declarations.first
    end

    it "exposes box sizing values as declaration wrappers" do
      {
        "content-box" => :content_box,
        "border-box" => :border_box,
      }.each do |css_value, expected_value|
        declaration = box_sizing_declaration(css_value)

        expect(declaration).to be_a(Yass::Declarations::BoxSizing)
        expect(declaration.value).to eq(expected_value)
      end
    end
  end

  describe "clear declarations" do
    def clear_declaration(value)
      sheet = Yass::Parser.parse(".x { clear: #{value}; }")
      sheet.rules.first.declarations.first
    end

    it "exposes clear values as declaration wrappers" do
      {
        "none" => :none,
        "left" => :left,
        "right" => :right,
        "both" => :both,
        "inline-start" => :inline_start,
        "inline-end" => :inline_end,
      }.each do |css_value, expected_value|
        declaration = clear_declaration(css_value)

        expect(declaration).to be_a(Yass::Declarations::Clear)
        expect(declaration.value).to eq(expected_value)
      end
    end
  end

  describe "direction declarations" do
    def direction_declaration(value)
      sheet = Yass::Parser.parse(".x { direction: #{value}; }")
      sheet.rules.first.declarations.first
    end

    it "exposes direction values as declaration wrappers" do
      {
        "ltr" => :ltr,
        "rtl" => :rtl,
      }.each do |css_value, expected_value|
        declaration = direction_declaration(css_value)

        expect(declaration).to be_a(Yass::Declarations::Direction)
        expect(declaration.value).to eq(expected_value)
      end
    end
  end

  describe "empty cells declarations" do
    def empty_cells_declaration(value)
      sheet = Yass::Parser.parse(".x { empty-cells: #{value}; }")
      sheet.rules.first.declarations.first
    end

    it "exposes empty cells values as declaration wrappers" do
      {
        "show" => :show,
        "hide" => :hide,
      }.each do |css_value, expected_value|
        declaration = empty_cells_declaration(css_value)

        expect(declaration).to be_a(Yass::Declarations::EmptyCells)
        expect(declaration.value).to eq(expected_value)
      end
    end
  end

  describe "flex basis declarations" do
    def flex_basis_declaration(value)
      sheet = Yass::Parser.parse(".x { flex-basis: #{value}; }")
      sheet.rules.first.declarations.first
    end

    it "exposes content and size flex-basis variants" do
      content_declaration = flex_basis_declaration("content")

      expect(content_declaration).to be_a(Yass::Declarations::FlexBasis)
      expect(content_declaration.value).to be_a(Yass::Declarations::FlexBasis::Content)

      size_declaration = flex_basis_declaration("25%")

      expect(size_declaration).to be_a(Yass::Declarations::FlexBasis)

      size_value = size_declaration.value
      expect(size_value).to be_a(Yass::Declarations::FlexBasis::Size)

      length_percentage = size_value.value
      expect(length_percentage).to be_a(Yass::Declarations::Size::LengthPercentage)

      percentage = length_percentage.value
      expect(percentage).to be_a(Yass::Declarations::Percentage)
      expect(percentage.value).to eq(0.25)
    end
  end

  describe "display declarations" do
    def display_declaration(value)
      sheet = Yass::Parser.parse(".x { display: #{value}; }")
      sheet.rules.first.declarations.first
    end

    it "exposes inside and outside display types" do
      declaration = display_declaration("inline flex")

      expect(declaration).to be_a(Yass::Declarations::Display)
      expect(declaration.inside).to eq(:flex)
      expect(declaration.outside).to eq(:inline)
      expect(declaration.list_item?).to eq(false)
      expect(declaration.inline_flow?).to eq(false)
      expect(declaration.item_container?).to eq(true)
      expect(declaration.line_participant?).to eq(false)
      expect(declaration.none?).to eq(false)
      expect(declaration.contents?).to eq(false)
    end

    it "exposes list-item display flags" do
      declaration = display_declaration("inline list-item")

      expect(declaration.inside).to eq(:flow)
      expect(declaration.outside).to eq(:inline)
      expect(declaration.list_item?).to eq(true)
      expect(declaration.inline_flow?).to eq(true)
      expect(declaration.line_participant?).to eq(true)
    end

    it "exposes the none keyword" do
      declaration = display_declaration("none")

      expect(declaration.inside).to eq(:none)
      expect(declaration.outside).to eq(:none)
      expect(declaration.none?).to eq(true)
      expect(declaration.contents?).to eq(false)
      expect(declaration.item_container?).to eq(false)
      expect(declaration.line_participant?).to eq(false)
    end
  end

  describe "cursor declarations" do
    def cursor_declaration(value)
      sheet = Yass::Parser.parse(".x { cursor: #{value}; }")
      sheet.rules.first.declarations.first
    end

    it "exposes cursor keyword and image values" do
      declaration = cursor_declaration('url("cursor.png") 12 24, pointer')

      expect(declaration).to be_a(Yass::Declarations::Cursor)
      expect(declaration.keyword).to eq(:pointer)

      images = declaration.images
      expect(images.size).to eq(1)
      expect(images.first).to be_a(Yass::Declarations::Cursor::Image)
      expect(images.first.hotspot?).to eq(true)
      expect(images.first.hotspot_x).to be_a(Yass::Declarations::Number)
      expect(images.first.hotspot_x.value).to eq(12.0)
      expect(images.first.hotspot_y.value).to eq(24.0)
      expect(images.first.image).to be_a(Yass::Declarations::Image::Url)
    end
  end

  describe "contain declarations" do
    def contain_declaration(value)
      sheet = Yass::Parser.parse(".x { contain: #{value}; }")
      sheet.rules.first.declarations.first
    end

    it "exposes contain values and predicates" do
      declaration = contain_declaration("layout style")

      expect(declaration).to be_a(Yass::Declarations::Contain)
      expect(declaration.values).to eq([:layout, :style])
      expect(declaration.layout?).to eq(true)
      expect(declaration.style?).to eq(true)
      expect(declaration.paint?).to eq(false)
      expect(declaration.content?).to eq(false)
      expect(declaration.strict?).to eq(false)
    end

    it "exposes strict as a canonical contain value" do
      declaration = contain_declaration("strict")

      expect(declaration.values).to eq([:strict])
      expect(declaration.strict?).to eq(true)
      expect(declaration.content?).to eq(true)
      expect(declaration.size?).to eq(true)
    end
  end

  describe "container name declarations" do
    def container_name_declaration(value)
      sheet = Yass::Parser.parse(".x { container-name: #{value}; }")
      sheet.rules.first.declarations.first
    end

    it "exposes named container values" do
      declaration = container_name_declaration("foo bar")

      expect(declaration).to be_a(Yass::Declarations::ContainerName)
      expect(declaration.values).to eq(["foo", "bar"])
      expect(declaration.none?).to eq(false)
    end

    it "exposes the none value" do
      declaration = container_name_declaration("none")

      expect(declaration).to be_a(Yass::Declarations::ContainerName)
      expect(declaration.values).to eq(["none"])
      expect(declaration.none?).to eq(true)
    end
  end

  describe "column gap declarations" do
    def column_gap_declaration(value)
      sheet = Yass::Parser.parse(".x { column-gap: #{value}; }")
      sheet.rules.first.declarations.first
    end

    it "exposes normal and length-percentage variants" do
      normal_declaration = column_gap_declaration("normal")

      expect(normal_declaration).to be_a(Yass::Declarations::ColumnGap)
      expect(normal_declaration.value).to be_a(Yass::Declarations::ColumnGap::Normal)

      length_declaration = column_gap_declaration("12px")

      expect(length_declaration).to be_a(Yass::Declarations::ColumnGap)

      length = length_declaration.value
      expect(length).to be_a(Yass::Declarations::ColumnGap::LengthPercentage)
      expect(length.value).to be_a(Yass::Declarations::Length::Absolute)
      expect(length.value.value).to eq(12.0)
      expect(length.value.unit).to eq(:px)

      percentage_declaration = column_gap_declaration("7%")

      percentage = percentage_declaration.value
      expect(percentage).to be_a(Yass::Declarations::ColumnGap::LengthPercentage)
      expect(percentage.value).to be_a(Yass::Declarations::Percentage)
      expect(percentage.value.value).to be_within(0.00001).of(0.07)
    end
  end

  describe "column span declarations" do
    def column_span_declaration(value)
      sheet = Yass::Parser.parse(".x { column-span: #{value}; }")
      sheet.rules.first.declarations.first
    end

    it "exposes all and none variants" do
      all_declaration = column_span_declaration("all")

      expect(all_declaration).to be_a(Yass::Declarations::ColumnSpan)
      expect(all_declaration.value).to eq(:all)

      none_declaration = column_span_declaration("none")

      expect(none_declaration).to be_a(Yass::Declarations::ColumnSpan)
      expect(none_declaration.value).to eq(:none)
    end
  end

  describe "column width declarations" do
    def column_width_declaration(value)
      sheet = Yass::Parser.parse(".x { column-width: #{value}; }")
      sheet.rules.first.declarations.first
    end

    it "exposes auto and length variants" do
      auto_declaration = column_width_declaration("auto")

      expect(auto_declaration).to be_a(Yass::Declarations::ColumnWidth)
      expect(auto_declaration.value).to be_a(Yass::Declarations::ColumnWidth::Auto)

      length_declaration = column_width_declaration("200px")

      expect(length_declaration).to be_a(Yass::Declarations::ColumnWidth)

      length = length_declaration.value
      expect(length).to be_a(Yass::Declarations::ColumnWidth::Length)
      expect(length.value).to be_a(Yass::Declarations::Length::Absolute)
      expect(length.value.value).to eq(200.0)
      expect(length.value.unit).to eq(:px)
    end
  end

  describe "color scheme declarations" do
    def color_scheme_declaration(value)
      sheet = Yass::Parser.parse(".x { color-scheme: #{value}; }")
      sheet.rules.first.declarations.first
    end

    it "exposes normal as a symbol" do
      declaration = color_scheme_declaration("normal")

      expect(declaration).to be_a(Yass::Declarations::ColorScheme)
      expect(declaration.values).to eq(["normal"])
    end

    it "exposes keyword and custom identifier values in canonical order" do
      declaration = color_scheme_declaration("only light dark sepia")

      expect(declaration).to be_a(Yass::Declarations::ColorScheme)
      expect(declaration.values).to eq(["light", "dark", "sepia", "only"])
    end
  end

  describe "clip-path declarations" do
    def clip_path_declaration(value)
      sheet = Yass::Parser.parse(".x { clip-path: #{value}; }")
      sheet.rules.first.declarations.first
    end

    it "exposes variant wrappers for none, shape, and geometry box" do
      none_declaration = clip_path_declaration("none")
      expect(none_declaration).to be_a(Yass::Declarations::ClipPath)
      expect(none_declaration.value).to be_a(Yass::Declarations::ClipPath::None)

      shape_declaration = clip_path_declaration("circle(40%) border-box")
      expect(shape_declaration).to be_a(Yass::Declarations::ClipPath)

      shape = shape_declaration.value
      expect(shape).to be_a(Yass::Declarations::ClipPath::Shape)
      expect(shape.shape).to be_a(Yass::Declarations::ClipPath::Circle)
      expect(shape.shape.position).to be_a(Yass::Declarations::ClipPath::PositionAuto)

      radius = shape.shape.radius
      expect(radius).to be_a(Yass::Declarations::ClipPath::ShapeRadiusLength)
      expect(radius.value).to be_a(Yass::Declarations::Percentage)
      expect(radius.value.value).to be_within(0.000001).of(0.4)

      expect(shape.reference_box).to be_a(Yass::Declarations::ClipPath::BorderBox)

      box_declaration = clip_path_declaration("content-box")
      box = box_declaration.value

      expect(box).to be_a(Yass::Declarations::ClipPath::Box)
      expect(box.reference_box).to be_a(Yass::Declarations::ClipPath::ContentBox)
    end

    it "exposes typed round radii and shape commands" do
      rounded_declaration = clip_path_declaration("inset(1px round 2px)")
      rounded_shape = rounded_declaration.value.shape

      expect(rounded_shape).to be_a(Yass::Declarations::ClipPath::Rect)

      inset_rect = rounded_shape.value
      expect(inset_rect).to be_a(Yass::Declarations::ClipPath::InsetRect)
      expect(inset_rect.round).to be_a(Yass::Declarations::ClipPath::BorderRadius)
      expect(inset_rect.round.top_left).to be_a(Yass::Declarations::ClipPath::BorderCornerRadius)

      path_declaration = clip_path_declaration('path("M 0 0 L 10 10")')
      path_or_shape = path_declaration.value.shape

      expect(path_or_shape).to be_a(Yass::Declarations::ClipPath::PathOrShape)
      path_function = path_or_shape.value
      expect(path_function).to be_a(Yass::Declarations::ClipPath::PathFunction)

      commands = path_function.commands
      expect(commands.size).to eq(2)
      expect(commands[0]).to be_a(Yass::Declarations::ClipPath::PathCommand)
      expect(commands[0].value).to be_a(Yass::Declarations::ClipPath::PathCommandMove)
      expect(commands[0].value.point).to be_a(Yass::Declarations::ClipPath::PathCommandEndPointToPosition)
      expect(commands[0].value.point.x).to eq(0.0)
      expect(commands[0].value.point.y).to eq(0.0)
    end
  end

  describe "clip declarations" do
    def clip_declaration(value)
      sheet = Yass::Parser.parse(".x { clip: #{value}; }")
      sheet.rules.first.declarations.first
    end

    it "exposes auto and rect clip wrappers" do
      auto_declaration = clip_declaration("auto")

      expect(auto_declaration).to be_a(Yass::Declarations::Clip)
      expect(auto_declaration.value).to be_a(Yass::Declarations::Clip::Auto)

      rect_declaration = clip_declaration("rect(1px, auto, 3px, 4px)")
      rect = rect_declaration.value

      expect(rect_declaration).to be_a(Yass::Declarations::Clip)
      expect(rect).to be_a(Yass::Declarations::Clip::Rect)

      expect(rect.top).to be_a(Yass::Declarations::Clip::Length)
      expect(rect.top.value).to be_a(Yass::Declarations::Length::Absolute)
      expect(rect.top.value.value).to eq(1.0)
      expect(rect.top.value.unit).to eq(:px)

      expect(rect.right).to be_a(Yass::Declarations::Clip::LengthAuto)

      expect(rect.bottom).to be_a(Yass::Declarations::Clip::Length)
      expect(rect.bottom.value).to be_a(Yass::Declarations::Length::Absolute)
      expect(rect.bottom.value.value).to eq(3.0)
      expect(rect.bottom.value.unit).to eq(:px)

      expect(rect.left).to be_a(Yass::Declarations::Clip::Length)
      expect(rect.left.value).to be_a(Yass::Declarations::Length::Absolute)
      expect(rect.left.value.value).to eq(4.0)
      expect(rect.left.value.unit).to eq(:px)
    end
  end

  describe "border image outset declarations" do
    def border_image_outset_declaration(value)
      sheet = Yass::Parser.parse(".x { border-image-outset: #{value}; }")
      sheet.rules.first.declarations.first
    end

    it "exposes border image outset sides as numbers or lengths" do
      declaration = border_image_outset_declaration("1 2px 3 4px")

      expect(declaration).to be_a(Yass::Declarations::BorderImageOutset)

      expect(declaration.top).to be_a(Yass::Declarations::Number)
      expect(declaration.top.value).to eq(1.0)

      expect(declaration.right).to be_a(Yass::Declarations::Length::Absolute)
      expect(declaration.right.value).to eq(2.0)
      expect(declaration.right.unit).to eq(:px)

      expect(declaration.bottom).to be_a(Yass::Declarations::Number)
      expect(declaration.bottom.value).to eq(3.0)

      expect(declaration.left).to be_a(Yass::Declarations::Length::Absolute)
      expect(declaration.left.value).to eq(4.0)
      expect(declaration.left.unit).to eq(:px)
    end
  end

  describe "border image repeat declarations" do
    def border_image_repeat_declaration(value)
      sheet = Yass::Parser.parse(".x { border-image-repeat: #{value}; }")
      sheet.rules.first.declarations.first
    end

    it "exposes horizontal and vertical border image repeat keywords" do
      declaration = border_image_repeat_declaration("repeat round")

      expect(declaration).to be_a(Yass::Declarations::BorderImageRepeat)
      expect(declaration.horizontal).to eq(:repeat)
      expect(declaration.vertical).to eq(:round)

      stretch = border_image_repeat_declaration("stretch")

      expect(stretch.horizontal).to eq(:stretch)
      expect(stretch.vertical).to eq(:stretch)
    end
  end

  describe "border image slice declarations" do
    def border_image_slice_declaration(value)
      sheet = Yass::Parser.parse(".x { border-image-slice: #{value}; }")
      sheet.rules.first.declarations.first
    end

    it "exposes border image slice sides and fill flag" do
      declaration = border_image_slice_declaration("10 20% 30 40% fill")

      expect(declaration).to be_a(Yass::Declarations::BorderImageSlice)
      expect(declaration.fill?).to eq(true)

      expect(declaration.top).to be_a(Yass::Declarations::Number)
      expect(declaration.top.value).to eq(10.0)

      expect(declaration.right).to be_a(Yass::Declarations::Percentage)
      expect(declaration.right.value).to be_within(0.00001).of(0.2)

      expect(declaration.bottom).to be_a(Yass::Declarations::Number)
      expect(declaration.bottom.value).to eq(30.0)

      expect(declaration.left).to be_a(Yass::Declarations::Percentage)
      expect(declaration.left.value).to be_within(0.00001).of(0.4)
    end
  end

  describe "border image width declarations" do
    def border_image_width_declaration(value)
      sheet = Yass::Parser.parse(".x { border-image-width: #{value}; }")
      sheet.rules.first.declarations.first
    end

    it "exposes border image width sides as numbers, lengths, or auto" do
      declaration = border_image_width_declaration("1 2px 3% auto")

      expect(declaration).to be_a(Yass::Declarations::BorderImageWidth)

      expect(declaration.top).to be_a(Yass::Declarations::Number)
      expect(declaration.top.value).to eq(1.0)

      expect(declaration.right).to be_a(Yass::Declarations::Size::LengthPercentage)
      expect(declaration.right.value).to be_a(Yass::Declarations::Length::Absolute)
      expect(declaration.right.value.value).to eq(2.0)
      expect(declaration.right.value.unit).to eq(:px)

      expect(declaration.bottom).to be_a(Yass::Declarations::Size::LengthPercentage)
      expect(declaration.bottom.value).to be_a(Yass::Declarations::Percentage)
      expect(declaration.bottom.value.value).to be_within(0.00001).of(0.03)

      expect(declaration.left).to be_a(Yass::Declarations::BorderImageWidth::Auto)
    end
  end

  describe "border width declarations" do
    def border_width_declaration(property, value)
      sheet = Yass::Parser.parse(".x { #{property}: #{value}; }")
      sheet.rules.first.declarations.first
    end

    it "exposes side border widths as declaration wrappers" do
      expectations = [
        ["border-top-width", "thin", Yass::Declarations::BorderTopWidth, :thin],
        ["border-right-width", "2px", Yass::Declarations::BorderRightWidth, "2px"],
        ["border-bottom-width", "medium", Yass::Declarations::BorderBottomWidth, :medium],
        ["border-left-width", "4px", Yass::Declarations::BorderLeftWidth, "4px"],
      ]

      expectations.each do |property, css_value, klass, expected_value|
        declaration = border_width_declaration(property, css_value)

        expect(declaration).to be_a(klass)
        expect(declaration.value).to eq(expected_value)
      end
    end
  end

  describe "container type declarations" do
    def container_type_declaration(value)
      sheet = Yass::Parser.parse(".x { container-type: #{value}; }")
      sheet.rules.first.declarations.first
    end

    it "exposes the normal value" do
      declaration = container_type_declaration("normal")

      expect(declaration).to be_a(Yass::Declarations::ContainerType)
      expect(declaration.values).to eq([:normal])
      expect(declaration.normal?).to eq(true)
      expect(declaration.inline_size?).to eq(false)
      expect(declaration.size?).to eq(false)
    end

    it "exposes the inline-size value" do
      declaration = container_type_declaration("inline-size")

      expect(declaration).to be_a(Yass::Declarations::ContainerType)
      expect(declaration.values).to eq([:inline_size])
      expect(declaration.normal?).to eq(false)
      expect(declaration.inline_size?).to eq(true)
      expect(declaration.size?).to eq(false)
    end

    it "exposes the size value" do
      declaration = container_type_declaration("size")

      expect(declaration).to be_a(Yass::Declarations::ContainerType)
      expect(declaration.values).to eq([:size])
      expect(declaration.normal?).to eq(false)
      expect(declaration.inline_size?).to eq(true)
      expect(declaration.size?).to eq(true)
    end
  end

  describe "custom declarations" do
    def custom_declaration(value)
      sheet = Yass::Parser.parse(".x { --theme: #{value}; }")
      sheet.rules.first.declarations.first
    end

    it "exposes custom property names and unparsed values" do
      declaration = custom_declaration("red")

      expect(declaration).to be_a(Yass::Declarations::Custom)
      expect(declaration.name).to eq("theme")

      value = declaration.value
      expect(value).to be_a(Yass::Declarations::Custom::Unparsed)
      expect(value.value).to eq("red")
    end

    it "exposes custom CSS-wide keyword values" do
      declaration = custom_declaration("initial")

      expect(declaration).to be_a(Yass::Declarations::Custom)

      value = declaration.value
      expect(value).to be_a(Yass::Declarations::Custom::CSSWideKeyword)
      expect(value.value).to eq(:initial)
    end
  end
end
