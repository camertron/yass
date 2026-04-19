# frozen_string_literal: true

require "spec_helper"

RSpec.describe "view-transition-name declaration" do
  def declaration_for(value)
    sheet = Yass::Parser.parse(".x { view-transition-name: #{value}; }")
    sheet.rules.first.declarations.first
  end

  it "exposes the none value" do
    declaration = declaration_for("none")

    expect(declaration).to be_a(Yass::Declarations::ViewTransitionName)
    expect(declaration.name).to eq("none")
  end

  it "exposes the match-element value" do
    declaration = declaration_for("match-element")

    expect(declaration).to be_a(Yass::Declarations::ViewTransitionName)
    expect(declaration.name).to eq("match-element")
  end

  it "exposes a custom identifier" do
    declaration = declaration_for("my-element")

    expect(declaration).to be_a(Yass::Declarations::ViewTransitionName)
    expect(declaration.name).to eq("my-element")
  end
end
