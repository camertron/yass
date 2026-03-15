use style::values::specified::Size;

#[magnus::wrap(class = "Yass::Declarations::MinBlockSize")]
pub struct YMinBlockSize {
  size: Size
}

impl YMinBlockSize {
  pub fn new(size: Size) -> Self {
    Self { size }
  }
}
