use style::values::{generics::{NonNegative, border::BorderSpacing}, specified::Length};

#[magnus::wrap(class = "Yass::Declarations::BorderSpacing")]
pub struct YBorderSpacing {
  specified_value: Box<BorderSpacing<NonNegative<Length>>>
}

impl YBorderSpacing {
  pub fn new(specified_value: Box<BorderSpacing<NonNegative<Length>>>) -> Self {
    Self { specified_value }
  }
}
