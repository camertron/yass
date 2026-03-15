use style::values::specified::NonNegativeLengthPercentage;

#[magnus::wrap(class = "Yass::Declarations::PaddingBottom")]
pub struct YPaddingBottom {
  non_negative_length_percentage: NonNegativeLengthPercentage
}

impl YPaddingBottom {
  pub fn new(non_negative_length_percentage: NonNegativeLengthPercentage) -> Self {
    Self { non_negative_length_percentage }
  }
}
