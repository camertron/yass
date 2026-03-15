use style::values::generics::color::GenericCaretColor;
use style::values::specified::color::Color;

#[magnus::wrap(class = "Yass::Declarations::CaretColor")]
pub struct YCaretColor {
  caret_color: GenericCaretColor<Color>
}

impl YCaretColor {
  pub fn new(caret_color: GenericCaretColor<Color>) -> Self {
    Self { caret_color }
  }
}
