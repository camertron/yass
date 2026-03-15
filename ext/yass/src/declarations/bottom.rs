use style::values::specified::Inset;

#[magnus::wrap(class = "Yass::Declarations::Bottom")]
pub struct YBottom {
  inset: Inset
}

impl YBottom {
  pub fn new(inset: Inset) -> Self {
    Self { inset }
  }
}
