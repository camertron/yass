use style::properties::longhands::animation_fill_mode::SpecifiedValue;

#[magnus::wrap(class = "Yass::Declarations::AnimationFillMode")]
pub struct YAnimationFillMode {
  specified_value: SpecifiedValue
}

impl YAnimationFillMode {
  pub fn new(specified_value: SpecifiedValue) -> Self {
    Self { specified_value }
  }
}
