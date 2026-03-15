use style::values::{generics::{NonNegative, border::BorderCornerRadius}, specified::LengthPercentage};

#[magnus::wrap(class = "Yass::Declarations::BorderStartStartRadius")]
pub struct YBorderStartStartRadius {
  specified_value: Box<BorderCornerRadius<NonNegative<LengthPercentage>>>
}

impl YBorderStartStartRadius {
  pub fn new(specified_value: Box<BorderCornerRadius<NonNegative<LengthPercentage>>>) -> Self {
    Self { specified_value }
  }
}
