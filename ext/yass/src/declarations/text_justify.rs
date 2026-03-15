use style::values::computed::TextJustify;

#[magnus::wrap(class = "Yass::Declarations::TextJustify")]
pub struct YTextJustify {
  text_justify: TextJustify
}

impl YTextJustify {
  pub fn new(text_justify: TextJustify) -> Self {
    Self { text_justify }
  }
}
