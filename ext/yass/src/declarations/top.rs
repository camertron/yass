use style::values::specified::Inset;

#[magnus::wrap(class = "Yass::Declarations::Top")]
pub struct YTop {
  inset: Inset
}

impl YTop {
  pub fn new(inset: Inset) -> Self {
    Self { inset }
  }
}
