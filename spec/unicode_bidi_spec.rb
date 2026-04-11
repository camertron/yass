# frozen_string_literal: true

require "spec_helper"

RSpec.describe "unicode-bidi declaration" do
  def declaration_for(value)
    sheet = Yass::Parser.parse(".x { unicode-bidi: #{value}; }")
    sheet.rules.first.declarations.first
  end

  it "exposes unicode-bidi values" do
    {
      "normal" => :normal,
      "embed" => :embed,
      "isolate" => :isolate,
      "bidi-override" => :bidi_override,
      "isolate-override" => :isolate_override,
      "plaintext" => :plaintext,
    }.each do |css_value, expected_value|
      declaration = declaration_for(css_value)

      expect(declaration).to be_a(Yass::Declarations::UnicodeBidi)
      expect(declaration.value).to eq(expected_value)
    end
  end
end
