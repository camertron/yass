# frozen_string_literal: true

require "spec_helper"

RSpec.describe "transition-behavior declaration" do
  def declaration_for(value)
    sheet = Yass::Parser.parse(".x { transition-behavior: #{value}; }")
    sheet.rules.first.declarations.first
  end

  it "exposes transition-behavior values" do
    declaration = declaration_for("normal")

    expect(declaration).to be_a(Yass::Declarations::TransitionBehavior)
    expect(declaration.values).to eq([:normal])
  end

  it "supports comma-separated transition-behavior values" do
    declaration = declaration_for("allow-discrete, normal")

    expect(declaration).to be_a(Yass::Declarations::TransitionBehavior)
    expect(declaration.values).to eq([:allow_discrete, :normal])
  end
end
