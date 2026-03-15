use style::properties::longhands::background_image::SpecifiedValue;

#[magnus::wrap(class = "Yass::Declarations::BackgroundImage")]
pub struct YBackgroundImage {
  specified_value: SpecifiedValue
}

impl YBackgroundImage {
  pub fn new(specified_value: SpecifiedValue) -> Self {
    Self { specified_value }
  }
}
