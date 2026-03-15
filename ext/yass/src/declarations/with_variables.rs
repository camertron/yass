use style::properties::VariableDeclaration;

#[magnus::wrap(class = "Yass::Declarations::WithVariables")]
pub struct YWithVariables {
  variable_declaration: VariableDeclaration
}

impl YWithVariables {
  pub fn new(variable_declaration: VariableDeclaration) -> Self {
    Self { variable_declaration }
  }
}
