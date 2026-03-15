use style::values::specified::Size;

#[magnus::wrap(class = "Yass::Declarations::InlineSize")]
pub struct YInlineSize {
  size: Size
}

impl YInlineSize {
  pub fn new(size: Size) -> Self {
    Self { size }
  }
}
