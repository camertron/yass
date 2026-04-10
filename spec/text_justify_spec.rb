# frozen_string_literal: true

require "spec_helper"

RSpec.describe "text-justify declaration" do
  def declaration_for(value)
    sheet = Yass::Parser.parse(".x { text-justify: #{value}; }")
    sheet.rules.first.declarations.first
  end

  it "exposes text-justify values" do
    {
      "auto" => :auto,
      "none" => :none,
      "inter-word" => :inter_word,
      "inter-character" => :inter_character,
      "distribute" => :inter_character,
    }.each do |css_value, expected_value|
      declaration = declaration_for(css_value)

      expect(declaration).to be_a(Yass::Declarations::TextJustify)
      expect(declaration.value).to eq(expected_value)
    end
  end
end
