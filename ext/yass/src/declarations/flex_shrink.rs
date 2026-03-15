use style::values::specified::NonNegativeNumber;

#[magnus::wrap(class = "Yass::Declarations::FlexShrink")]
pub struct YFlexShrink {
  non_negative_number: NonNegativeNumber
}

impl YFlexShrink {
  pub fn new(non_negative_number: NonNegativeNumber) -> Self {
    Self { non_negative_number }
  }
}
