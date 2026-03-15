use style::values::specified::NonNegativeNumber;

#[magnus::wrap(class = "Yass::Declarations::FlexGrow")]
pub struct YFlexGrow {
  non_negative_number: NonNegativeNumber
}

impl YFlexGrow {
  pub fn new(non_negative_number: NonNegativeNumber) -> Self {
    Self { non_negative_number }
  }
}
