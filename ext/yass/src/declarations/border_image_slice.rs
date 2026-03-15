use style::values::{generics::{NonNegative, border::BorderImageSlice}, specified::NumberOrPercentage};

#[magnus::wrap(class = "Yass::Declarations::BorderImageSlice")]
pub struct YBorderImageSlice {
  specified_value: Box<BorderImageSlice<NonNegative<NumberOrPercentage>>>
}

impl YBorderImageSlice {
  pub fn new(specified_value: Box<BorderImageSlice<NonNegative<NumberOrPercentage>>>) -> Self {
    Self { specified_value }
  }
}
