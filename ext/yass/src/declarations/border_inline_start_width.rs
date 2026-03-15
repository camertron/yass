use style::values::specified::BorderSideWidth;

#[magnus::wrap(class = "Yass::Declarations::BorderInlineStartWidth")]
pub struct YBorderInlineStartWidth {
  border_side_width: BorderSideWidth
}

impl YBorderInlineStartWidth {
  pub fn new(border_side_width: BorderSideWidth) -> Self {
    Self { border_side_width }
  }
}
