use style::values::{generics::{NonNegative, border::BorderCornerRadius}, specified::LengthPercentage};

#[magnus::wrap(class = "Yass::Declarations::BorderBottomRightRadius")]
pub struct YBorderBottomRightRadius {
  specified_value: Box<BorderCornerRadius<NonNegative<LengthPercentage>>>
}

impl YBorderBottomRightRadius {
  pub fn new(specified_value: Box<BorderCornerRadius<NonNegative<LengthPercentage>>>) -> Self {
    Self { specified_value }
  }
}
