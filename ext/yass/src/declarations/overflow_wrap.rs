use style::values::computed::OverflowWrap;

#[magnus::wrap(class = "Yass::Declarations::OverflowWrap")]
pub struct YOverflowWrap {
  overflow_wrap: OverflowWrap
}

impl YOverflowWrap {
  pub fn new(overflow_wrap: OverflowWrap) -> Self {
    Self { overflow_wrap }
  }
}
