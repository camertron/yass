use style::properties::longhands::transition_behavior::SpecifiedValue;

#[magnus::wrap(class = "Yass::Declarations::TransitionBehavior")]
pub struct YTransitionBehavior {
  specified_value: SpecifiedValue
}

impl YTransitionBehavior {
  pub fn new(specified_value: SpecifiedValue) -> Self {
    Self { specified_value }
  }
}
