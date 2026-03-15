use style::values::{generics::{NonNegative, border::BorderImageSideWidth, rect::Rect}, specified::{LengthPercentage, Number}};

#[magnus::wrap(class = "Yass::Declarations::BorderImageWidth")]
pub struct YBorderImageWidth {
  specified_value: Box<Rect<BorderImageSideWidth<NonNegative<LengthPercentage>, NonNegative<Number>>>>
}

impl YBorderImageWidth {
  pub fn new(specified_value: Box<Rect<BorderImageSideWidth<NonNegative<LengthPercentage>, NonNegative<Number>>>>) -> Self {
    Self { specified_value }
  }
}
