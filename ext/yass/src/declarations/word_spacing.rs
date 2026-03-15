use style::values::specified::WordSpacing;

#[magnus::wrap(class = "Yass::Declarations::WordSpacing")]
pub struct YWordSpacing {
  word_spacing: WordSpacing
}

impl YWordSpacing {
  pub fn new(word_spacing: WordSpacing) -> Self {
    Self { word_spacing }
  }
}
