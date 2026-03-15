use style::values::specified::Margin;

#[magnus::wrap(class = "Yass::Declarations::MarginTop")]
pub struct YMarginTop {
  margin: Margin
}

impl YMarginTop {
  pub fn new(margin: Margin) -> Self {
    Self { margin }
  }
}
