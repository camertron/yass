# frozen_string_literal: true

require "spec_helper"

RSpec.describe(Yass) do
  describe "@media rules" do
    def parse_first_rule(css)
      Yass::Parser.parse(css).rules.first
    end

    it "parses @media into a media rule" do
      rule = parse_first_rule(<<~CSS)
        @media screen {
          .x { color: red; }
        }
      CSS

      expect(rule).to be_a(Yass::MediaRule)
      expect(rule.kind).to eq(:media_rule)
      expect(rule.media_queries.size).to eq(1)
    end

    it "exposes media query qualifier and feature expression" do
      rule = parse_first_rule(<<~CSS)
        @media not screen and (max-width: 600px) {
          .x { color: red; }
        }
      CSS

      media_query = rule.media_queries.first

      expect(media_query).to be_a(Yass::MediaQuery)
      expect(media_query.kind).to eq(:media_query)
      expect(media_query.qualifier).to eq(:not)

      expect(media_query.media_type).to be_a(Yass::MediaType::Concrete)
      expect(media_query.media_type.value).to eq("screen")

      expect(media_query.query_condition).to be_a(Yass::MediaQuery::QueryCondition::FeatureExpression)
      expect(media_query.query_condition.value).to eq("(max-width: 600px)")
    end

    it "exposes nested rules inside @media" do
      rule = parse_first_rule(<<~CSS)
        @media screen {
          .x { color: red; }
          .y { opacity: 0.5; }
        }
      CSS

      expect(rule.rules.size).to eq(2)
      expect(rule.rules[0]).to be_a(Yass::StyleRule)
      expect(rule.rules[1]).to be_a(Yass::StyleRule)

      expect(rule.rules[0].declarations.first).to be_a(Yass::Declarations::Color)
      expect(rule.rules[1].declarations.first).to be_a(Yass::Declarations::Opacity)
    end
  end
end
