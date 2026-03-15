use style::computed_values::list_style_position::T;

#[magnus::wrap(class = "Yass::Declarations::ListStylePosition")]
pub struct YListStylePosition {
  specified_value: T
}

impl YListStylePosition {
  pub fn new(specified_value: T) -> Self {
    Self { specified_value }
  }
}
