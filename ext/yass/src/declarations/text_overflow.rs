use style::values::computed::TextOverflow;

#[magnus::wrap(class = "Yass::Declarations::TextOverflow")]
pub struct YTextOverflow {
  specified_value: Box<TextOverflow>
}

impl YTextOverflow {
  pub fn new(specified_value: Box<TextOverflow>) -> Self {
    Self { specified_value }
  }
}
