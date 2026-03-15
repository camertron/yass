use style::values::specified::GridTemplateComponent;

#[magnus::wrap(class = "Yass::Declarations::GridTemplateColumns")]
pub struct YGridTemplateColumns {
  grid_template_component: GridTemplateComponent
}

impl YGridTemplateColumns {
  pub fn new(grid_template_component: GridTemplateComponent) -> Self {
    Self { grid_template_component }
  }
}
