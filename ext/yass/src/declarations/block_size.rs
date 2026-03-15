use style::values::specified::Size;

#[magnus::wrap(class = "Yass::Declarations::BlockSize")]
pub struct YBlockSize {
  size: Size
}

impl YBlockSize {
  pub fn new(size: Size) -> Self {
    Self { size }
  }
}
