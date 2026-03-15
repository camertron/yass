use style::values::specified::Size;

#[magnus::wrap(class = "Yass::Declarations::Height")]
pub struct YHeight {
  size: Size
}

impl YHeight {
  pub fn new(size: Size) -> Self {
    Self { size }
  }
}
