use style::values::specified::Color;

#[magnus::wrap(class = "Yass::Declarations::OutlineColor")]
pub struct YOutlineColor {
  color: Color
}

impl YOutlineColor {
  pub fn new(color: Color) -> Self {
    Self { color }
  }
}
