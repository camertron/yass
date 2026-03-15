use style::values::computed::Overflow;

#[magnus::wrap(class = "Yass::Declarations::OverflowInline")]
pub struct YOverflowInline {
  overflow: Overflow
}

impl YOverflowInline {
  pub fn new(overflow: Overflow) -> Self {
    Self { overflow }
  }
}
