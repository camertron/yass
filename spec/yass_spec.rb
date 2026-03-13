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
end
