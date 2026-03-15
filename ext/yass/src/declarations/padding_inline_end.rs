use style::values::specified::NonNegativeLengthPercentage;

#[magnus::wrap(class = "Yass::Declarations::PaddingInlineEnd")]
pub struct YPaddingInlineEnd {
  non_negative_length_percentage: NonNegativeLengthPercentage
}

impl YPaddingInlineEnd {
  pub fn new(non_negative_length_percentage: NonNegativeLengthPercentage) -> Self {
    Self { non_negative_length_percentage }
  }
}
