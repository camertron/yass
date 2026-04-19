require "yass"

sheet = Yass::Parser.parse(<<~CSS)
  @media screen and (max-width: 600px) {
    .container {
      flex-direction: column;
    }
  }
CSS

# this is the whole @media { ... } rule
first_rule = sheet.rules.first
puts first_rule.class  # => Yass::MediaRule

# this is the "screen and ..." part
first_query = first_rule.media_queries.first
puts first_query.media_type.value       # => "screen"
puts first_query.query_condition.value  # => "(max-width: 600px)"

# Sadly Stylo doesn't give us access to the full parse tree for the
# query condition, so the best we can do is emit the unparsed string.

# this is the .container rule
first_nested_rule = first_rule.rules.first
first_selector = first_nested_rule.selectors.first.children.first
puts first_selector.value  # => "container"

# this is the flex-direction declaration
first_declaration = first_nested_rule.declarations.first
puts first_declaration.kind   # => :flex_direction
puts first_declaration.value  # => :column
