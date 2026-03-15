use style::properties::CustomDeclaration;

#[magnus::wrap(class = "Yass::Declarations::Custom")]
pub struct YCustom {
  custom_declaration: CustomDeclaration
}

impl YCustom {
  pub fn new(custom_declaration: CustomDeclaration) -> Self {
    Self { custom_declaration }
  }
}
