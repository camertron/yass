use style::values::specified::LetterSpacing;

#[magnus::wrap(class = "Yass::Declarations::LetterSpacing")]
pub struct YLetterSpacing {
  letter_spacing: LetterSpacing
}

impl YLetterSpacing {
  pub fn new(letter_spacing: LetterSpacing) -> Self {
    Self { letter_spacing }
  }
}
