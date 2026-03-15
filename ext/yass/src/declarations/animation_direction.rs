use style::properties::longhands::animation_direction::SpecifiedValue;

#[magnus::wrap(class = "Yass::Declarations::AnimationDirection")]
pub struct YAnimationDirection {
  specified_value: SpecifiedValue
}

impl YAnimationDirection {
  pub fn new(specified_value: SpecifiedValue) -> Self {
    Self { specified_value }
  }
}
