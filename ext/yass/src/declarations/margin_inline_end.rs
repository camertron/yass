use style::values::specified::Margin;

#[magnus::wrap(class = "Yass::Declarations::MarginInlineEnd")]
pub struct YMarginInlineEnd {
  margin: Margin
}

impl YMarginInlineEnd {
  pub fn new(margin: Margin) -> Self {
    Self { margin }
  }
}
