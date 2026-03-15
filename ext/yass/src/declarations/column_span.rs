use style::computed_values::column_span::T;

#[magnus::wrap(class = "Yass::Declarations::ColumnSpan")]
pub struct YColumnSpan {
  specified_value: T
}

impl YColumnSpan {
  pub fn new(specified_value: T) -> Self {
    Self { specified_value }
  }
}
