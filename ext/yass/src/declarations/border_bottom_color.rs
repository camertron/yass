use style::values::specified::Color;

#[magnus::wrap(class = "Yass::Declarations::BorderBottomColor")]
pub struct YBorderBottomColor {
  color: Color
}

impl YBorderBottomColor {
  pub fn new(color: Color) -> Self {
    Self { color }
  }
}
