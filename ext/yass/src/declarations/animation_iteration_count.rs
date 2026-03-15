use style::properties::longhands::animation_iteration_count::SpecifiedValue;

#[magnus::wrap(class = "Yass::Declarations::AnimationIterationCount")]
pub struct YAnimationIterationCount {
  specified_value: SpecifiedValue
}

impl YAnimationIterationCount {
  pub fn new(specified_value: SpecifiedValue) -> Self {
    Self { specified_value }
  }
}
