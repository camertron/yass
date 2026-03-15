use style::properties::longhands::background_attachment::SpecifiedValue;

#[magnus::wrap(class = "Yass::Declarations::BackgroundAttachment")]
pub struct YBackgroundAttachment {
  specified_value: SpecifiedValue
}

impl YBackgroundAttachment {
  pub fn new(specified_value: SpecifiedValue) -> Self {
    Self { specified_value }
  }
}
