use style::values::specified::NonNegativeLengthPercentage;

#[magnus::wrap(class = "Yass::Declarations::PaddingRight")]
pub struct YPaddingRight {
  non_negative_length_percentage: NonNegativeLengthPercentage
}

impl YPaddingRight {
  pub fn new(non_negative_length_percentage: NonNegativeLengthPercentage) -> Self {
    Self { non_negative_length_percentage }
  }
}
