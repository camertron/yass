require "yass"
require "pp"

sheet = Yass::Parser.parse(<<~CSS)
  h1 {
    align-items: center;
    display: flex;
    position: absolute;
  }
CSS

# this is the whole h1 { ... } rule
first_rule = sheet.rules.first
puts first_rule.class  # => Yass::StyleRule

# this is the h1 selector
first_selector = first_rule.selectors.first.children.first
puts first_selector.value  # => "h1"

# this is all the declarations inside the h1 rule
declarations = first_rule.declarations

# this is the first align-items declaration
declaration = declarations.first
puts declaration.kind   # => :align_items
puts declaration.class  # => Yass::Declarations::AlignItems
puts declaration.value  # => :center

# this will turn the entire stylesheet into a JSON-compatible
# data structure (only arrays, hashes, strings, etc) and
# pretty-print it
pp sheet.to_h
