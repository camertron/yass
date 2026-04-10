require_relative "spec_helper"

RSpec.describe(Yass) do
  describe "object-position declarations" do
    it "exposes horizontal and vertical position components" do
      stylesheet = Yass::Parser.parse("div { object-position: 50% 50%; }")
      decl = stylesheet.rules[0].declarations[0]
      expect(decl).to be_a(Yass::Declarations::ObjectPosition)
      expect(decl.horizontal).not_to be_nil
      expect(decl.vertical).not_to be_nil
    end

    it "exposes left top position" do
      stylesheet = Yass::Parser.parse("div { object-position: left top; }")
      decl = stylesheet.rules[0].declarations[0]
      expect(decl.horizontal).not_to be_nil
      expect(decl.vertical).not_to be_nil
    end

    it "exposes center center position" do
      stylesheet = Yass::Parser.parse("div { object-position: center center; }")
      decl = stylesheet.rules[0].declarations[0]
      expect(decl.horizontal).not_to be_nil
      expect(decl.vertical).not_to be_nil
    end

    it "exposes right bottom position" do
      stylesheet = Yass::Parser.parse("div { object-position: right bottom; }")
      decl = stylesheet.rules[0].declarations[0]
      expect(decl.horizontal).not_to be_nil
      expect(decl.vertical).not_to be_nil
    end
  end
end
