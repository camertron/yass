use style::values::computed::GridTemplateAreas;

#[magnus::wrap(class = "Yass::Declarations::GridTemplateAreas")]
pub struct YGridTemplateAreas {
  grid_template_areas: GridTemplateAreas
}

impl YGridTemplateAreas {
  pub fn new(grid_template_areas: GridTemplateAreas) -> Self {
    Self { grid_template_areas }
  }
}
