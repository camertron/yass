use style::values::specified::Size;

#[magnus::wrap(class = "Yass::Declarations::Width")]
pub struct YWidth {
  size: Size
}

impl YWidth {
  pub fn new(size: Size) -> Self {
    Self { size }
  }
}
