use style::values::specified::Color;

#[magnus::wrap(class = "Yass::Declarations::BorderTopColor")]
pub struct YBorderTopColor {
  color: Color
}

impl YBorderTopColor {
  pub fn new(color: Color) -> Self {
    Self { color }
  }
}
