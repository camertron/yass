use style::computed_values::isolation::T;

#[magnus::wrap(class = "Yass::Declarations::Isolation")]
pub struct YIsolation {
  specified_value: T
}

impl YIsolation {
  pub fn new(specified_value: T) -> Self {
    Self { specified_value }
  }
}
