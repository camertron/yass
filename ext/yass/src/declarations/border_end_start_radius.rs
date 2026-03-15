use style::values::{generics::{NonNegative, border::BorderCornerRadius}, specified::LengthPercentage};

#[magnus::wrap(class = "Yass::Declarations::BorderEndStartRadius")]
pub struct YBorderEndStartRadius {
  specified_value: Box<BorderCornerRadius<NonNegative<LengthPercentage>>>
}

impl YBorderEndStartRadius {
  pub fn new(specified_value: Box<BorderCornerRadius<NonNegative<LengthPercentage>>>) -> Self {
    Self { specified_value }
  }
}
