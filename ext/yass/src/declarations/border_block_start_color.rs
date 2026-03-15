use style::values::specified::Color;

#[magnus::wrap(class = "Yass::Declarations::BorderBlockStartColor")]
pub struct YBorderBlockStartColor {
  color: Color
}

impl YBorderBlockStartColor {
  pub fn new(color: Color) -> Self {
    Self { color }
  }
}
