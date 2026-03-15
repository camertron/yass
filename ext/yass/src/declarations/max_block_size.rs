use style::values::specified::MaxSize;

#[magnus::wrap(class = "Yass::Declarations::MaxBlockSize")]
pub struct YMaxBlockSize {
  max_size: MaxSize
}

impl YMaxBlockSize {
  pub fn new(max_size: MaxSize) -> Self {
    Self { max_size }
  }
}
