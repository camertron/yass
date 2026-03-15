use style::values::computed::WritingModeProperty;

#[magnus::wrap(class = "Yass::Declarations::WritingMode")]
pub struct YWritingMode {
  writing_mode_property: WritingModeProperty
}

impl YWritingMode {
  pub fn new(writing_mode_property: WritingModeProperty) -> Self {
    Self { writing_mode_property }
  }
}
