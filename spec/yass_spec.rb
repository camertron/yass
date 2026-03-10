# frozen_string_literal: true

RSpec.describe Yass do
  it "has a version number" do
    expect(Yass::VERSION).not_to be nil
  end

  it "returns the selector count for style rules" do
    sheet = Yass::Parser.parse(<<~CSS)
      h1, h2 {
        color: red;
      }
    CSS

    expect(sheet.rules.map(&:selector_len)).to eq([2])
  end
end
