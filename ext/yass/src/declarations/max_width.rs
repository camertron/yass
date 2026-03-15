use style::values::specified::MaxSize;

#[magnus::wrap(class = "Yass::Declarations::MaxWidth")]
pub struct YMaxWidth {
  max_size: MaxSize
}

impl YMaxWidth {
  pub fn new(max_size: MaxSize) -> Self {
    Self { max_size }
  }
}
