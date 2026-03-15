use style::properties::longhands::backdrop_filter::SpecifiedValue;

#[magnus::wrap(class = "Yass::Declarations::BackdropFilter")]
pub struct YBackdropFilter {
  specified_value: SpecifiedValue
}

impl YBackdropFilter {
  pub fn new(specified_value: SpecifiedValue) -> Self {
    Self { specified_value }
  }
}
