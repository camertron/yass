use style::values::specified::Inset;

#[magnus::wrap(class = "Yass::Declarations::InsetBlockEnd")]
pub struct YInsetBlockEnd {
  inset: Inset
}

impl YInsetBlockEnd {
  pub fn new(inset: Inset) -> Self {
    Self { inset }
  }
}
