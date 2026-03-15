use style::values::computed::TextTransform;

#[magnus::wrap(class = "Yass::Declarations::TextTransform")]
pub struct YTextTransform {
  text_transform: TextTransform
}

impl YTextTransform {
  pub fn new(text_transform: TextTransform) -> Self {
    Self { text_transform }
  }
}
