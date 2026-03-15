use style::computed_values::visibility::T;

#[magnus::wrap(class = "Yass::Declarations::Visibility")]
pub struct YVisibility {
  specified_value: T
}

impl YVisibility {
  pub fn new(specified_value: T) -> Self {
    Self { specified_value }
  }
}
