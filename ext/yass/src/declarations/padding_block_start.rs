use style::values::specified::NonNegativeLengthPercentage;

#[magnus::wrap(class = "Yass::Declarations::PaddingBlockStart")]
pub struct YPaddingBlockStart {
  non_negative_length_percentage: NonNegativeLengthPercentage
}

impl YPaddingBlockStart {
  pub fn new(non_negative_length_percentage: NonNegativeLengthPercentage) -> Self {
    Self { non_negative_length_percentage }
  }
}
