use style::values::specified::ColorPropertyValue;

#[magnus::wrap(class = "Yass::Declarations::Color")]
pub struct YColor {
  color_property_value: ColorPropertyValue
}

impl YColor {
  pub fn new(color_property_value: ColorPropertyValue) -> Self {
    Self { color_property_value }
  }
}
