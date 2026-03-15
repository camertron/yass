use style::values::specified::FontFamily;

#[magnus::wrap(class = "Yass::Declarations::FontFamily")]
pub struct YFontFamily {
  font_family: FontFamily
}

impl YFontFamily {
  pub fn new(font_family: FontFamily) -> Self {
    Self { font_family }
  }
}
