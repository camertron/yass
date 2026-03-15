use style::values::specified::FontStyle;

#[magnus::wrap(class = "Yass::Declarations::FontStyle")]
pub struct YFontStyle {
  font_style: FontStyle
}

impl YFontStyle {
  pub fn new(font_style: FontStyle) -> Self {
    Self { font_style }
  }
}
