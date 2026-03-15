use style::values::{generics::{ClipRect, ClipRectOrAuto, length::LengthPercentageOrAuto}, specified::Length};

#[magnus::wrap(class = "Yass::Declarations::Clip")]
pub struct YClip {
  specified_value: Box<ClipRectOrAuto<ClipRect<LengthPercentageOrAuto<Length>>>>
}

impl YClip {
  pub fn new(specified_value: Box<ClipRectOrAuto<ClipRect<LengthPercentageOrAuto<Length>>>>) -> Self {
    Self { specified_value }
  }
}
