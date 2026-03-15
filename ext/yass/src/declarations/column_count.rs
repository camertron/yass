use style::values::{generics::{GreaterThanOrEqualToOne, column::ColumnCount}, specified::Integer};

#[magnus::wrap(class = "Yass::Declarations::ColumnCount")]
pub struct YColumnCount {
  column_count: ColumnCount<GreaterThanOrEqualToOne<Integer>>
}

impl YColumnCount {
  pub fn new(column_count: ColumnCount<GreaterThanOrEqualToOne<Integer>>) -> Self {
    Self { column_count }
  }
}
