use style::computed_values::direction::T;

#[magnus::wrap(class = "Yass::Declarations::Direction")]
pub struct YDirection {
  specified_value: T
}

impl YDirection {
  pub fn new(specified_value: T) -> Self {
    Self { specified_value }
  }
}
