use style::values::{generics::text::GenericTextIndent, specified::LengthPercentage};

#[magnus::wrap(class = "Yass::Declarations::TextIndent")]
pub struct YTextIndent {
  text_indent: GenericTextIndent<LengthPercentage>
}

impl YTextIndent {
  pub fn new(text_indent: GenericTextIndent<LengthPercentage>) -> Self {
    Self { text_indent }
  }
}
