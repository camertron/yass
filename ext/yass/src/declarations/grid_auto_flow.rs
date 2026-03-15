use style::values::computed::GridAutoFlow;

#[magnus::wrap(class = "Yass::Declarations::GridAutoFlow")]
pub struct YGridAutoFlow {
  grid_auto_flow: GridAutoFlow
}

impl YGridAutoFlow {
  pub fn new(grid_auto_flow: GridAutoFlow) -> Self {
    Self { grid_auto_flow }
  }
}
