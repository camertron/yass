use style::values::computed::WordBreak;

#[magnus::wrap(class = "Yass::Declarations::WordBreak")]
pub struct YWordBreak {
  word_break: WordBreak
}

impl YWordBreak {
  pub fn new(word_break: WordBreak) -> Self {
    Self { word_break }
  }
}
