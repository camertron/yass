use style::values::computed::Overflow;

#[magnus::wrap(class = "Yass::Declarations::OverflowX")]
pub struct YOverflowX {
  overflow: Overflow
}

impl YOverflowX {
  pub fn new(overflow: Overflow) -> Self {
    Self { overflow }
  }
}
