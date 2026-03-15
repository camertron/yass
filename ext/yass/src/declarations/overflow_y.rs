use style::values::computed::Overflow;

#[magnus::wrap(class = "Yass::Declarations::OverflowY")]
pub struct YOverflowY {
  overflow: Overflow
}

impl YOverflowY {
  pub fn new(overflow: Overflow) -> Self {
    Self { overflow }
  }
}
