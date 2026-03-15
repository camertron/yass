use style::properties::longhands::background_position_x::SpecifiedValue;

#[magnus::wrap(class = "Yass::Declarations::BackgroundPositionX")]
pub struct YBackgroundPositionX {
  specified_value: SpecifiedValue
}

impl YBackgroundPositionX {
  pub fn new(specified_value: SpecifiedValue) -> Self {
    Self { specified_value }
  }
}
