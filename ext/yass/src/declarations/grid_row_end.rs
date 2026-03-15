use style::values::specified::GridLine;

#[magnus::wrap(class = "Yass::Declarations::GridRowEnd")]
pub struct YGridRowEnd {
  grid_line: GridLine
}

impl YGridRowEnd {
  pub fn new(grid_line: GridLine) -> Self {
    Self { grid_line }
  }
}
