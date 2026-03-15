use style::values::specified::Inset;

#[magnus::wrap(class = "Yass::Declarations::Left")]
pub struct YLeft {
  inset: Inset
}

impl YLeft {
  pub fn new(inset: Inset) -> Self {
    Self { inset }
  }
}
