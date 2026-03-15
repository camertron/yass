use style::values::specified::NonNegativeLengthPercentage;

#[magnus::wrap(class = "Yass::Declarations::PaddingBlockEnd")]
pub struct YPaddingBlockEnd {
  non_negative_length_percentage: NonNegativeLengthPercentage
}

impl YPaddingBlockEnd {
  pub fn new(non_negative_length_percentage: NonNegativeLengthPercentage) -> Self {
    Self { non_negative_length_percentage }
  }
}
