use style::computed_values::empty_cells::T;

#[magnus::wrap(class = "Yass::Declarations::EmptyCells")]
pub struct YEmptyCells {
  specified_value: T
}

impl YEmptyCells {
  pub fn new(specified_value: T) -> Self {
    Self { specified_value }
  }
}
