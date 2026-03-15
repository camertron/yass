use style::values::specified::BorderSideWidth;

#[magnus::wrap(class = "Yass::Declarations::BorderBottomWidth")]
pub struct YBorderBottomWidth {
  border_side_width: BorderSideWidth
}

impl YBorderBottomWidth {
  pub fn new(border_side_width: BorderSideWidth) -> Self {
    Self { border_side_width }
  }
}
