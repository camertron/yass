use style::values::specified::Color;

#[magnus::wrap(class = "Yass::Declarations::BackgroundColor")]
pub struct YBackgroundColor {
  color: Color
}

impl YBackgroundColor {
  pub fn new(color: Color) -> Self {
    Self { color }
  }
}
