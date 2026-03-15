use style::properties::longhands::box_shadow::SpecifiedValue;

#[magnus::wrap(class = "Yass::Declarations::BoxShadow")]
pub struct YBoxShadow {
  specified_value: SpecifiedValue
}

impl YBoxShadow {
  pub fn new(specified_value: SpecifiedValue) -> Self {
    Self { specified_value }
  }
}
