use style::values::specified::NonNegativeLengthPercentage;

#[magnus::wrap(class = "Yass::Declarations::PaddingTop")]
pub struct YPaddingTop {
  non_negative_length_percentage: NonNegativeLengthPercentage
}

impl YPaddingTop {
  pub fn new(non_negative_length_percentage: NonNegativeLengthPercentage) -> Self {
    Self { non_negative_length_percentage }
  }
}
