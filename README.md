# Yass

Yass is a Ruby wrapper around the Stylo CSS engine, the parser behind the Firefox and Servo browsers developed by Mozilla.

## Rationale

I needed a CSS parser in Ruby but couldn't find one that could handle nested CSS rules. Stylo was born out of that need, and aims to be a complete CSS parser for Ruby, covering the entire CSS spec.

Stylo is itself under active development, and Yass does not yet wrap the entire Stylo API. If Yass doesn't support a feature you need, consider adding the missing functionality and submitting a pull request.

## Installation

Install the gem and add to the application's Gemfile by executing:

```bash
bundle add yass-css
```

If bundler is not being used to manage dependencies, install the gem by executing:

```bash
gem install yass-css
```

## Usage

There are two main ways to use Yass: by navigating the object hierarchy by hand, or by using the visitor pattern.

By "navigating the object hierarchy," I just mean "calling methods on objects." Here's an example.

```ruby
require "yass"

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
```

The entire object hierarchy can be discovered by dropping into a console session (via `bin/console`) and playing around.

### The Visitor Pattern

Yass also comes with a `Yass::Visitor` class that implements the [visitor pattern](https://en.wikipedia.org/wiki/Visitor_pattern).

Some people love the visitor pattern, and some people seem to hate it. If you're in the second camp, feel free to skip this section.

Visitor classes contain `visit_*` methods that the library calls in-order as it traverses the object hierarchy contained within a parsed stylesheet. The methods the visitor supports can be examined by reading the code in lib/yass/visitor.rb.

Let's write a visitor class that prints all the class names in the given stylesheet:

```ruby
class MyVisitor < Yass::Visitor
  def visit_selector_klass(node)
    # node is an instance of Yass::Selector::Klass
    puts node.value
  end
end
```

Now we can use our visitor to visit a stylesheet:

```ruby
sheet = Yass::Parser.parse(<<~CSS)
  .left {
    float: left;
  }

  .right {
    float: right;
  }
CSS

visitor = MyVisitor.new

# this will result in "left" and "right" being printed to stdout
visitor.visit(sheet)
```

## Examples

For more examples, see the examples/ directory.

## Development

After checking out the repo, run `bundle install` to install dependencies. Then, run `bundle exec rake` to compile the native extension and run the tests. You can also run `bin/console` for an interactive prompt that will allow you to experiment.

### Running Tests

Simply execute:

```bash
bundle exec rake
```

This will compile the native extension and run the tests immediately afterwards. To compile only, run:

```bash
bundle exec rake compile
```

To run the tests only, run:

```bash
bundle exec rake spec
```

To run the full test suite (much slower) that attempts to parse the ~33k example CSS files from the [Web Platform Test (WPT) suite](https://github.com/web-platform-tests/wpt), run:

```ruby
bundle exec rake spec:full
```

## Releasing

Releasing Yass is done by creating a new release on GitHub. Doing so will trigger a special CI job capable of building the native extension for all supported plaforms and publishing a "fat" .gem file to rubygems.org.

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/camertron/yass. This project is intended to be a safe, welcoming space for collaboration, and contributors are expected to adhere to the [code of conduct](https://github.com/camertron/yass/blob/main/CODE_OF_CONDUCT.md).

## License

The gem is available as open source under the terms of the [MIT License](https://opensource.org/licenses/MIT).

## Code of Conduct

Everyone interacting in the Yass project's codebases, issue trackers, chat rooms and mailing lists is expected to follow the [code of conduct](https://github.com/camertron/yass/blob/main/CODE_OF_CONDUCT.md).
