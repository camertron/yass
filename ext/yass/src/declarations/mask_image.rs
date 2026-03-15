use style::properties::longhands::mask_image::SpecifiedValue;

#[magnus::wrap(class = "Yass::Declarations::MaskImage")]
pub struct YMaskImage {
  specified_value: SpecifiedValue
}

impl YMaskImage {
  pub fn new(specified_value: SpecifiedValue) -> Self {
    Self { specified_value }
  }
}
