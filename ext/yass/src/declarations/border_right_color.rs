use style::values::specified::Color;

#[magnus::wrap(class = "Yass::Declarations::BorderRightColor")]
pub struct YBorderRightColor {
  color: Color
}

impl YBorderRightColor {
  pub fn new(color: Color) -> Self {
    Self { color }
  }
}
