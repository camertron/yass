use style::computed_values::object_fit::T;

#[magnus::wrap(class = "Yass::Declarations::ObjectFit")]
pub struct YObjectFit {
  specified_value: T
}

impl YObjectFit {
  pub fn new(specified_value: T) -> Self {
    Self { specified_value }
  }
}
