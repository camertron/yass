use style::values::specified::Color;

#[magnus::wrap(class = "Yass::Declarations::BorderInlineEndColor")]
pub struct YBorderInlineEndColor {
  color: Color
}

impl YBorderInlineEndColor {
  pub fn new(color: Color) -> Self {
    Self { color }
  }
}
