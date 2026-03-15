use style::values::specified::Size;

#[magnus::wrap(class = "Yass::Declarations::MinWidth")]
pub struct YMinWidth {
  size: Size
}

impl YMinWidth {
  pub fn new(size: Size) -> Self {
    Self { size }
  }
}
