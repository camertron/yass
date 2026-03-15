use style::values::specified::NonNegativeLengthPercentage;

#[magnus::wrap(class = "Yass::Declarations::PaddingLeft")]
pub struct YPaddingLeft {
  non_negative_length_percentage: NonNegativeLengthPercentage
}

impl YPaddingLeft {
  pub fn new(non_negative_length_percentage: NonNegativeLengthPercentage) -> Self {
    Self { non_negative_length_percentage }
  }
}
