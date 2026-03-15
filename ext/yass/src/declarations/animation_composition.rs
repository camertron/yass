use style::properties::longhands::animation_composition::SpecifiedValue;

#[magnus::wrap(class = "Yass::Declarations::AnimationComposition")]
pub struct YAnimationComposition {
  specified_value: SpecifiedValue
}

impl YAnimationComposition {
  pub fn new(specified_value: SpecifiedValue) -> Self {
    Self { specified_value }
  }
}
