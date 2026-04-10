# frozen_string_literal: true

require "spec_helper"

RSpec.describe(Yass) do
  describe "mix blend mode declarations" do
    def mix_blend_mode_declaration(value)
      sheet = Yass::Parser.parse(".x { mix-blend-mode: #{value}; }")
      sheet.rules.first.declarations.first
    end

    it "exposes mix blend mode values as declaration wrappers" do
      {
        "normal" => :normal,
        "color-dodge" => :color_dodge,
        "hard-light" => :hard_light,
        "luminosity" => :luminosity,
      }.each do |css_value, expected_value|
        declaration = mix_blend_mode_declaration(css_value)

        expect(declaration).to be_a(Yass::Declarations::MixBlendMode)
        expect(declaration.value).to eq(expected_value)
      end
    end
  end
end
