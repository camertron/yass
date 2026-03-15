use style::values::specified::GridLine;

#[magnus::wrap(class = "Yass::Declarations::GridColumnEnd")]
pub struct YGridColumnEnd {
  grid_line: GridLine
}

impl YGridColumnEnd {
  pub fn new(grid_line: GridLine) -> Self {
    Self { grid_line }
  }
}
