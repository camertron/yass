use style::values::specified::LineHeight;

#[magnus::wrap(class = "Yass::Declarations::LineHeight")]
pub struct YLineHeight {
  line_height: LineHeight
}

impl YLineHeight {
  pub fn new(line_height: LineHeight) -> Self {
    Self { line_height }
  }
}
