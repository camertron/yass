use style::values::{generics::transform::Scale, specified::Number};

#[magnus::wrap(class = "Yass::Declarations::Scale")]
pub struct YScale {
  specified_value: Box<Scale<Number>>
}

impl YScale {
  pub fn new(specified_value: Box<Scale<Number>>) -> Self {
    Self { specified_value }
  }
}
