use style::values::specified::BorderSideWidth;

#[magnus::wrap(class = "Yass::Declarations::BorderTopWidth")]
pub struct YBorderTopWidth {
  border_side_width: BorderSideWidth
}

impl YBorderTopWidth {
  pub fn new(border_side_width: BorderSideWidth) -> Self {
    Self { border_side_width }
  }
}
