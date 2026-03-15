use style::values::specified::FontStretch;

#[magnus::wrap(class = "Yass::Declarations::FontStretch")]
pub struct YFontStretch {
  font_stretch: FontStretch
}

impl YFontStretch {
  pub fn new(font_stretch: FontStretch) -> Self {
    Self { font_stretch }
  }
}
