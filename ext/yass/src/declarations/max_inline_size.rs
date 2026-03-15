use style::values::specified::MaxSize;

#[magnus::wrap(class = "Yass::Declarations::MaxInlineSize")]
pub struct YMaxInlineSize {
  max_size: MaxSize
}

impl YMaxInlineSize {
  pub fn new(max_size: MaxSize) -> Self {
    Self { max_size }
  }
}
