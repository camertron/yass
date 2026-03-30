# frozen_string_literal: true

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
    expect(second_selector_children[0].kind).to eq(:class)
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

    expect(values[0]).to be_a(Yass::Declarations::BackgroundSizeCover)
    expect(values[1]).to be_a(Yass::Declarations::BackgroundSizeContain)

    explicit_size = values[2]
    expect(explicit_size).to be_a(Yass::Declarations::BackgroundSizeExplicitSize)
    expect(explicit_size.width).to be_a(Yass::Declarations::Percentage)
    expect(explicit_size.height).to be_a(Yass::Declarations::BackgroundSizeAuto)
  end
end
