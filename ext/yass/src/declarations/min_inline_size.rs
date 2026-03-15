use style::values::specified::Size;

#[magnus::wrap(class = "Yass::Declarations::MinInlineSize")]
pub struct YMinInlineSize {
  size: Size
}

impl YMinInlineSize {
  pub fn new(size: Size) -> Self {
    Self { size }
  }
}
