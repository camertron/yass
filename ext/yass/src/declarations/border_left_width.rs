use style::values::specified::BorderSideWidth;

#[magnus::wrap(class = "Yass::Declarations::BorderLeftWidth")]
pub struct YBorderLeftWidth {
  border_side_width: BorderSideWidth
}

impl YBorderLeftWidth {
  pub fn new(border_side_width: BorderSideWidth) -> Self {
    Self { border_side_width }
  }
}
