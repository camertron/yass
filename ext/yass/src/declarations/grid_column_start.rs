use style::values::specified::GridLine;

#[magnus::wrap(class = "Yass::Declarations::GridColumnStart")]
pub struct YGridColumnStart {
  grid_line: GridLine
}

impl YGridColumnStart {
  pub fn new(grid_line: GridLine) -> Self {
    Self { grid_line }
  }
}
