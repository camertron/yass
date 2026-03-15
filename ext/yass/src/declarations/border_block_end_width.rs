use style::values::specified::BorderSideWidth;

#[magnus::wrap(class = "Yass::Declarations::BorderBlockEndWidth")]
pub struct YBorderBlockEndWidth {
  border_side_width: BorderSideWidth
}

impl YBorderBlockEndWidth {
  pub fn new(border_side_width: BorderSideWidth) -> Self {
    Self { border_side_width }
  }
}
