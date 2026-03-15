use style::properties::longhands::transition_timing_function::SpecifiedValue;

#[magnus::wrap(class = "Yass::Declarations::TransitionTimingFunction")]
pub struct YTransitionTimingFunction {
  specified_value: SpecifiedValue
}

impl YTransitionTimingFunction {
  pub fn new(specified_value: SpecifiedValue) -> Self {
    Self { specified_value }
  }
}
