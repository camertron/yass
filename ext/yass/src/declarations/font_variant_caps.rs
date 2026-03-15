use style::computed_values::font_variant_caps::T;

#[magnus::wrap(class = "Yass::Declarations::FontVariantCaps")]
pub struct YFontVariantCaps {
  specified_value: T
}

impl YFontVariantCaps {
  pub fn new(specified_value: T) -> Self {
    Self { specified_value }
  }
}
