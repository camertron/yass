use style::values::specified::Margin;

#[magnus::wrap(class = "Yass::Declarations::MarginBottom")]
pub struct YMarginBottom {
  margin: Margin
}

impl YMarginBottom {
  pub fn new(margin: Margin) -> Self {
    Self { margin }
  }
}
