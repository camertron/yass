use style::values::{generics::{NonNegative, length::LengthPercentageOrNormal}, specified::LengthPercentage};

#[magnus::wrap(class = "Yass::Declarations::ColumnGap")]
pub struct YColumnGap {
  non_negative_length_percentage_or_normal: LengthPercentageOrNormal<NonNegative<LengthPercentage>>
}

impl YColumnGap {
  pub fn new(non_negative_length_percentage_or_normal: LengthPercentageOrNormal<NonNegative<LengthPercentage>>) -> Self {
    Self { non_negative_length_percentage_or_normal }
  }
}
