use style::values::specified::TextAlign;

#[magnus::wrap(class = "Yass::Declarations::TextAlign")]
pub struct YTextAlign {
  text_align: TextAlign
}

impl YTextAlign {
  pub fn new(text_align: TextAlign) -> Self {
    Self { text_align }
  }
}
