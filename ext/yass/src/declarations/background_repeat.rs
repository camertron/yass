use style::properties::longhands::background_repeat::SpecifiedValue;

#[magnus::wrap(class = "Yass::Declarations::BackgroundRepeat")]
pub struct YBackgroundRepeat {
  specified_value: SpecifiedValue
}

impl YBackgroundRepeat {
  pub fn new(specified_value: SpecifiedValue) -> Self {
    Self { specified_value }
  }
}
