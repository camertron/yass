use style::properties::longhands::background_origin::SpecifiedValue;

#[magnus::wrap(class = "Yass::Declarations::BackgroundOrigin")]
pub struct YBackgroundOrigin {
  specified_value: SpecifiedValue
}

impl YBackgroundOrigin {
  pub fn new(specified_value: SpecifiedValue) -> Self {
    Self { specified_value }
  }
}
