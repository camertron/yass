require "yass"

sheet = Yass::Parser.parse(<<~CSS)
  @font-face {
    font-family: "Bitstream Vera Serif Bold";
    src: url("https://mdn.github.io/shared-assets/fonts/FiraSans-Regular.woff2");
  }
CSS

# this is the whole @font-face { ... } rule
first_rule = sheet.rules.first
puts first_rule.class  # => Yass::FontFaceRule

family = first_rule.family
puts family.name  # => "Bitstream Vera Serif Bold"

first_source = first_rule.sources.first
puts first_source.class          # => Yass::Font::Source::Url
puts first_source.specified_url  # => "https://mdn.github.io/shared-assets/fonts/FiraSans-Regular.woff2"
