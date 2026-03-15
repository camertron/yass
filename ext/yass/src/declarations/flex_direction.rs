use style::computed_values::flex_direction::T;

#[magnus::wrap(class = "Yass::Declarations::FlexDirection")]
pub struct YFlexDirection {
  specified_value: T
}

impl YFlexDirection {
  pub fn new(specified_value: T) -> Self {
    Self { specified_value }
  }
}
