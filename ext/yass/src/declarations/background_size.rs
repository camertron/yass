use style::properties::longhands::background_size::SpecifiedValue;

#[magnus::wrap(class = "Yass::Declarations::BackgroundSize")]
pub struct YBackgroundSize {
  specified_value: SpecifiedValue
}

impl YBackgroundSize {
  pub fn new(specified_value: SpecifiedValue) -> Self {
    Self { specified_value }
  }
}
