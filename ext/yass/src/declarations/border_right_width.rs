use style::values::specified::BorderSideWidth;

#[magnus::wrap(class = "Yass::Declarations::BorderRightWidth")]
pub struct YBorderRightWidth {
  border_side_width: BorderSideWidth
}

impl YBorderRightWidth {
  pub fn new(border_side_width: BorderSideWidth) -> Self {
    Self { border_side_width }
  }
}
