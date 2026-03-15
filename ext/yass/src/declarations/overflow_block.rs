use style::values::computed::Overflow;

#[magnus::wrap(class = "Yass::Declarations::OverflowBlock")]
pub struct YOverflowBlock {
  overflow: Overflow
}

impl YOverflowBlock {
  pub fn new(overflow: Overflow) -> Self {
    Self { overflow }
  }
}
