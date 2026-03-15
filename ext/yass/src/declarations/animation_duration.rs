use style::properties::longhands::animation_duration::SpecifiedValue;

#[magnus::wrap(class = "Yass::Declarations::AnimationDuration")]
pub struct YAnimationDuration {
  specified_value: SpecifiedValue
}

impl YAnimationDuration {
  pub fn new(specified_value: SpecifiedValue) -> Self {
    Self { specified_value }
  }
}
