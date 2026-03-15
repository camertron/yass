use style::values::specified::Color;

#[magnus::wrap(class = "Yass::Declarations::BorderBlockEndColor")]
pub struct YBorderBlockEndColor {
  color: Color
}

impl YBorderBlockEndColor {
  pub fn new(color: Color) -> Self {
    Self { color }
  }
}
