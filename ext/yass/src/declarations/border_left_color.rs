use style::values::specified::Color;

#[magnus::wrap(class = "Yass::Declarations::BorderLeftColor")]
pub struct YBorderLeftColor {
  color: Color
}

impl YBorderLeftColor {
  pub fn new(color: Color) -> Self {
    Self { color }
  }
}
