use style::values::computed::FontSynthesis;

#[magnus::wrap(class = "Yass::Declarations::FontSynthesisWeight")]
pub struct YFontSynthesisWeight {
  font_synthesis: FontSynthesis
}

impl YFontSynthesisWeight {
  pub fn new(font_synthesis: FontSynthesis) -> Self {
    Self { font_synthesis }
  }
}
