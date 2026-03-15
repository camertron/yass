use style::properties::longhands::box_sizing::SpecifiedValue;

#[magnus::wrap(class = "Yass::Declarations::BoxSizing")]
pub struct YBoxSizing {
  specified_value: SpecifiedValue
}

impl YBoxSizing {
  pub fn new(specified_value: SpecifiedValue) -> Self {
    Self { specified_value }
  }
}
