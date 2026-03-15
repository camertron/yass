use style::properties::longhands::animation_timeline::SpecifiedValue;

#[magnus::wrap(class = "Yass::Declarations::AnimationTimeline")]
pub struct YAnimationTimeline {
  specified_value: SpecifiedValue
}

impl YAnimationTimeline {
  pub fn new(specified_value: SpecifiedValue) -> Self {
    Self { specified_value }
  }
}
