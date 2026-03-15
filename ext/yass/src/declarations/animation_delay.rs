use style::properties::longhands::animation_delay::SpecifiedValue;

#[magnus::wrap(class = "Yass::Declarations::AnimationDelay")]
pub struct YAnimationDelay {
  specified_value: SpecifiedValue
}

impl YAnimationDelay {
  pub fn new(specified_value: SpecifiedValue) -> Self {
    Self { specified_value }
  }
}
