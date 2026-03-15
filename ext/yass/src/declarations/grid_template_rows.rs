use style::values::specified::GridTemplateComponent;

#[magnus::wrap(class = "Yass::Declarations::GridTemplateRows")]
pub struct YGridTemplateRows {
  grid_template_component: GridTemplateComponent
}

impl YGridTemplateRows {
  pub fn new(grid_template_component: GridTemplateComponent) -> Self {
    Self { grid_template_component }
  }
}
