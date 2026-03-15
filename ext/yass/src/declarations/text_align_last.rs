use style::values::computed::TextAlignLast;

#[magnus::wrap(class = "Yass::Declarations::TextAlignLast")]
pub struct YTextAlignLast {
  text_align_last: TextAlignLast
}

impl YTextAlignLast {
  pub fn new(text_align_last: TextAlignLast) -> Self {
    Self { text_align_last }
  }
}
