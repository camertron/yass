require "yass"

sheet = Yass::Parser.parse(<<~CSS)
  h1 {
    align-items: center;
    display: flex;
    position: absolute;
  }

  .left {
    float: left;
  }

  .right {
    float: right;
  }

  #thing {
    visibility: hidden;
  }
CSS

class MyVisitor < Yass::Visitor
  def visit_selector_local_name(node)
    # prints "h1"
    puts node.value
  end

  def visit_selector_klass(node)
    # prints "left" and "right"
    puts node.value
  end

  def visit_selector_id(node)
    # prints "thing"
    puts node.value
  end
end

visitor = MyVisitor.new
visitor.visit(sheet)
