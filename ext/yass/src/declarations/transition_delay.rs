use style::properties::longhands::transition_delay::SpecifiedValue;

#[magnus::wrap(class = "Yass::Declarations::TransitionDelay")]
pub struct YTransitionDelay {
  specified_value: SpecifiedValue
}

impl YTransitionDelay {
  pub fn new(specified_value: SpecifiedValue) -> Self {
    Self { specified_value }
  }
}
