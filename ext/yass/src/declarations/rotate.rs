use style::values::{generics::transform::Rotate, specified::{Angle, Number}};

#[magnus::wrap(class = "Yass::Declarations::Rotate")]
pub struct YRotate {
  specified_value: Box<Rotate<Number, Angle>>
}

impl YRotate {
  pub fn new(specified_value: Box<Rotate<Number, Angle>>) -> Self {
    Self { specified_value }
  }
}
