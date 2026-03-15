use style::values::specified::FontVariationSettings;

#[magnus::wrap(class = "Yass::Declarations::FontVariationSettings")]
pub struct YFontVariationSettings {
  font_variation_settings: FontVariationSettings
}

impl YFontVariationSettings {
  pub fn new(font_variation_settings: FontVariationSettings) -> Self {
    Self { font_variation_settings }
  }
}
