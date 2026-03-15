use style::values::computed::FontLanguageOverride;

#[magnus::wrap(class = "Yass::Declarations::FontLanguageOverride")]
pub struct YFontLanguageOverride {
  font_language_override: FontLanguageOverride
}

impl YFontLanguageOverride {
  pub fn new(font_language_override: FontLanguageOverride) -> Self {
    Self { font_language_override }
  }
}
