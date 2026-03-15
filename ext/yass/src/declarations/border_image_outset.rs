use style::values::{generics::{NonNegative, length::LengthOrNumber, rect::Rect}, specified::{Length, Number}};

#[magnus::wrap(class = "Yass::Declarations::BorderImageOutset")]
pub struct YBorderImageOutset {
  specified_value: Box<Rect<LengthOrNumber<NonNegative<Length>, NonNegative<Number>>>>
}

impl YBorderImageOutset {
  pub fn new(specified_value: Box<Rect<LengthOrNumber<NonNegative<Length>, NonNegative<Number>>>>) -> Self {
    Self { specified_value }
  }
}
