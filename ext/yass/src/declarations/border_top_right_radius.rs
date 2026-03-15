use style::values::{generics::{NonNegative, border::BorderCornerRadius}, specified::LengthPercentage};

#[magnus::wrap(class = "Yass::Declarations::BorderTopRightRadius")]
pub struct YBorderTopRightRadius {
  specified_value: Box<BorderCornerRadius<NonNegative<LengthPercentage>>>
}

impl YBorderTopRightRadius {
  pub fn new(specified_value: Box<BorderCornerRadius<NonNegative<LengthPercentage>>>) -> Self {
    Self { specified_value }
  }
}
