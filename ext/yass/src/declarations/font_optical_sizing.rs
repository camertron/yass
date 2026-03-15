use style::computed_values::font_optical_sizing::T;

#[magnus::wrap(class = "Yass::Declarations::FontOpticalSizing")]
pub struct YFontOpticalSizing {
  specified_value: T
}

impl YFontOpticalSizing {
  pub fn new(specified_value: T) -> Self {
    Self { specified_value }
  }
}
