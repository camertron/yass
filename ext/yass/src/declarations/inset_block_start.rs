use style::values::specified::Inset;

#[magnus::wrap(class = "Yass::Declarations::InsetBlockStart")]
pub struct YInsetBlockStart {
  inset: Inset
}

impl YInsetBlockStart {
  pub fn new(inset: Inset) -> Self {
    Self { inset }
  }
}
