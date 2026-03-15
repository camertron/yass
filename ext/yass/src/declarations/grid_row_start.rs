use style::values::specified::GridLine;

#[magnus::wrap(class = "Yass::Declarations::GridRowStart")]
pub struct YGridRowStart {
  grid_line: GridLine
}

impl YGridRowStart {
  pub fn new(grid_line: GridLine) -> Self {
    Self { grid_line }
  }
}
