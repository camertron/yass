use style::values::specified::FontSize;

#[magnus::wrap(class = "Yass::Declarations::FontSize")]
pub struct YFontSize {
  font_size: FontSize
}

impl YFontSize {
  pub fn new(font_size: FontSize) -> Self {
    Self { font_size }
  }
}
