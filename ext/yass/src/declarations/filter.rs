use style::properties::longhands::filter::SpecifiedValue;

#[magnus::wrap(class = "Yass::Declarations::Filter")]
pub struct YFilter {
  specified_value: SpecifiedValue
}

impl YFilter {
  pub fn new(specified_value: SpecifiedValue) -> Self {
    Self { specified_value }
  }
}
