use style::values::specified::Color;

#[magnus::wrap(class = "Yass::Declarations::BorderInlineStartColor")]
pub struct YBorderInlineStartColor {
  color: Color
}

impl YBorderInlineStartColor {
  pub fn new(color: Color) -> Self {
    Self { color }
  }
}
