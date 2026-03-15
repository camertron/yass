use style::properties::longhands::transition_property::SpecifiedValue;

#[magnus::wrap(class = "Yass::Declarations::TransitionProperty")]
pub struct YTransitionProperty {
  specified_value: SpecifiedValue
}

impl YTransitionProperty {
  pub fn new(specified_value: SpecifiedValue) -> Self {
    Self { specified_value }
  }
}
