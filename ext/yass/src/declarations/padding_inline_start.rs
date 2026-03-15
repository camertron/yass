use style::values::specified::NonNegativeLengthPercentage;

#[magnus::wrap(class = "Yass::Declarations::PaddingInlineStart")]
pub struct YPaddingInlineStart {
  non_negative_length_percentage: NonNegativeLengthPercentage
}

impl YPaddingInlineStart {
  pub fn new(non_negative_length_percentage: NonNegativeLengthPercentage) -> Self {
    Self { non_negative_length_percentage }
  }
}
