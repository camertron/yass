use style::properties::longhands::animation_play_state::SpecifiedValue;

#[magnus::wrap(class = "Yass::Declarations::AnimationPlayState")]
pub struct YAnimationPlayState {
  specified_value: SpecifiedValue
}

impl YAnimationPlayState {
  pub fn new(specified_value: SpecifiedValue) -> Self {
    Self { specified_value }
  }
}
