use style::values::{generics::{NonNegative, length::LengthPercentageOrAuto}, specified::Length};

#[magnus::wrap(class = "Yass::Declarations::ColumnWidth")]
pub struct YColumnWidth {
  non_negative_length_or_auto: LengthPercentageOrAuto<NonNegative<Length>>
}

impl YColumnWidth {
  pub fn new(non_negative_length_or_auto: LengthPercentageOrAuto<NonNegative<Length>>) -> Self {
    Self { non_negative_length_or_auto }
  }
}
