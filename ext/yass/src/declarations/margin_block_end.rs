use style::values::specified::Margin;

#[magnus::wrap(class = "Yass::Declarations::MarginBlockEnd")]
pub struct YMarginBlockEnd {
  margin: Margin
}

impl YMarginBlockEnd {
  pub fn new(margin: Margin) -> Self {
    Self { margin }
  }
}
