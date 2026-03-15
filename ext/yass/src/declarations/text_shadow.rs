use style::properties::longhands::text_shadow::SpecifiedValue;

#[magnus::wrap(class = "Yass::Declarations::TextShadow")]
pub struct YTextShadow {
  specified_value: SpecifiedValue
}

impl YTextShadow {
  pub fn new(specified_value: SpecifiedValue) -> Self {
    Self { specified_value }
  }
}
