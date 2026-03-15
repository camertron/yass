use style::properties::longhands::background_position_y::SpecifiedValue;

#[magnus::wrap(class = "Yass::Declarations::BackgroundPositionY")]
pub struct YBackgroundPositionY {
  specified_value: SpecifiedValue
}

impl YBackgroundPositionY {
  pub fn new(specified_value: SpecifiedValue) -> Self {
    Self { specified_value }
  }
}
