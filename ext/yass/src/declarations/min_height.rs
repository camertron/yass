use style::values::specified::Size;

#[magnus::wrap(class = "Yass::Declarations::MinHeight")]
pub struct YMinHeight {
  size: Size
}

impl YMinHeight {
  pub fn new(size: Size) -> Self {
    Self { size }
  }
}
