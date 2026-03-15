use style::values::{generics::{NonNegative, border::BorderCornerRadius}, specified::LengthPercentage};

#[magnus::wrap(class = "Yass::Declarations::BorderStartEndRadius")]
pub struct YBorderStartEndRadius {
  specified_value: Box<BorderCornerRadius<NonNegative<LengthPercentage>>>
}

impl YBorderStartEndRadius {
  pub fn new(specified_value: Box<BorderCornerRadius<NonNegative<LengthPercentage>>>) -> Self {
    Self { specified_value }
  }
}
