use style::values::specified::FontWeight;

#[magnus::wrap(class = "Yass::Declarations::FontWeight")]
pub struct YFontWeight {
  font_weight: FontWeight
}

impl YFontWeight {
  pub fn new(font_weight: FontWeight) -> Self {
    Self { font_weight }
  }
}
