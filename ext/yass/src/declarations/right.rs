use style::values::specified::Inset;

#[magnus::wrap(class = "Yass::Declarations::Right")]
pub struct YRight {
  inset: Inset
}

impl YRight {
  pub fn new(inset: Inset) -> Self {
    Self { inset }
  }
}
