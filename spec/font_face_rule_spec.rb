# frozen_string_literal: true

require "spec_helper"

RSpec.describe(Yass) do
  describe "@font-face rules" do
    def parse_first_rule(css)
      Yass::Parser.parse(css).rules.first
    end

    it "parses @font-face into a font face rule" do
      rule = parse_first_rule(<<~CSS)
        @font-face {
          font-family: Test;
          src: url(test.woff2);
        }
      CSS

      expect(rule).to be_a(Yass::FontFaceRule)
      expect(rule.kind).to eq(:font_face_rule)
    end

    it "exposes family and source entries" do
      rule = parse_first_rule(<<~CSS)
        @font-face {
          font-family: Test;
          src: local("Open Sans"), url(test.woff2) format("woff2");
        }
      CSS

      expect(rule.family).to be_a(Yass::Font::Family::Name)
      expect(rule.family.name).to eq("Test")
      expect(rule.family.syntax).to eq(:identifiers)

      expect(rule.sources.size).to eq(2)

      local_source = rule.sources[0]
      expect(local_source).to be_a(Yass::Font::Source::Local)
      expect(local_source.family_name.name).to eq("Open Sans")
      expect(local_source.family_name.syntax).to eq(:quoted)

      url_source = rule.sources[1]
      expect(url_source).to be_a(Yass::Font::Source::Url)
      expect(url_source.specified_url).to end_with("/test.woff2")
      expect(url_source.format_hint).to be_a(Yass::Font::SourceFormat::String)
      expect(url_source.format_hint.value).to eq("woff2")
    end

    it "exposes hint and style descriptors" do
      rule = parse_first_rule(<<~CSS)
        @font-face {
          font-family: Test;
          src: url(test.woff2) format(woff2);
          font-style: italic;
        }
      CSS

      format_hint = rule.sources.first.format_hint
      expect(format_hint).to be_a(Yass::Font::SourceFormat::Keyword)
      expect(format_hint.value).to eq(:woff2)

      expect(rule.style).to be_a(Yass::FontStyle::Italic)
      expect(rule.style.kind).to eq(:italic)
    end
  end
end
