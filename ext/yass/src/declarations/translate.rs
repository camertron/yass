use style::values::{generics::transform::Translate, specified::{Length, LengthPercentage}};

#[magnus::wrap(class = "Yass::Declarations::Translate")]
pub struct YTranslate {
  specified_value: Box<Translate<LengthPercentage, Length>>
}

impl YTranslate {
  pub fn new(specified_value: Box<Translate<LengthPercentage, Length>>) -> Self {
    Self { specified_value }
  }
}
