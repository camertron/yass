use style::values::specified::BorderSideWidth;

#[magnus::wrap(class = "Yass::Declarations::OutlineWidth")]
pub struct YOutlineWidth {
  border_side_width: BorderSideWidth
}

impl YOutlineWidth {
  pub fn new(border_side_width: BorderSideWidth) -> Self {
    Self { border_side_width }
  }
}
