use style::values::specified::BorderSideWidth;

#[magnus::wrap(class = "Yass::Declarations::BorderBlockStartWidth")]
pub struct YBorderBlockStartWidth {
  border_side_width: BorderSideWidth
}

impl YBorderBlockStartWidth {
  pub fn new(border_side_width: BorderSideWidth) -> Self {
    Self { border_side_width }
  }
}
