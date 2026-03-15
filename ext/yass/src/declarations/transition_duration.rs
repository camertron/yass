use style::properties::longhands::transition_duration::SpecifiedValue;

#[magnus::wrap(class = "Yass::Declarations::TransitionDuration")]
pub struct YTransitionDuration {
  specified_value: SpecifiedValue
}

impl YTransitionDuration {
  pub fn new(specified_value: SpecifiedValue) -> Self {
    Self { specified_value }
  }
}
