# frozen_string_literal: true

require "spec_helper"

RSpec.describe(Yass) do
  describe "visibility declarations" do
    def visibility_declaration(value)
      sheet = Yass::Parser.parse(".x { visibility: #{value}; }")
      sheet.rules.first.declarations.first
    end

    it "exposes visibility values as declaration wrappers" do
      {
        "visible" => :visible,
        "hidden" => :hidden,
        "collapse" => :collapse,
      }.each do |css_value, expected_value|
        declaration = visibility_declaration(css_value)

        expect(declaration).to be_a(Yass::Declarations::Visibility)
        expect(declaration.value).to eq(expected_value)
      end
    end
  end
end
