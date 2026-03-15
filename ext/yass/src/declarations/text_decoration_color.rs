use style::values::specified::Color;

#[magnus::wrap(class = "Yass::Declarations::TextDecorationColor")]
pub struct YTextDecorationColor {
  color: Color
}

impl YTextDecorationColor {
  pub fn new(color: Color) -> Self {
    Self { color }
  }
}
