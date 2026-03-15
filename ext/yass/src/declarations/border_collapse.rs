use style::properties::longhands::border_collapse::SpecifiedValue;

#[magnus::wrap(class = "Yass::Declarations::BorderCollapse")]
pub struct YBorderCollapse {
  specified_value: SpecifiedValue
}

impl YBorderCollapse {
  pub fn new(specified_value: SpecifiedValue) -> Self {
    Self { specified_value }
  }
}
