use style::values::specified::Inset;

#[magnus::wrap(class = "Yass::Declarations::InsetInlineStart")]
pub struct YInsetInlineStart {
  inset: Inset
}

impl YInsetInlineStart {
  pub fn new(inset: Inset) -> Self {
    Self { inset }
  }
}
