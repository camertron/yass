use style::values::specified::MaxSize;

#[magnus::wrap(class = "Yass::Declarations::MaxHeight")]
pub struct YMaxHeight {
  max_size: MaxSize
}

impl YMaxHeight {
  pub fn new(max_size: MaxSize) -> Self {
    Self { max_size }
  }
}
