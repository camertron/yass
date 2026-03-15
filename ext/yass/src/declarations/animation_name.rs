use style::properties::longhands::animation_name::SpecifiedValue;

#[magnus::wrap(class = "Yass::Declarations::AnimationName")]
pub struct YAnimationName {
  specified_value: SpecifiedValue
}

impl YAnimationName {
  pub fn new(specified_value: SpecifiedValue) -> Self {
    Self { specified_value }
  }
}
