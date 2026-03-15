use style::values::specified::BorderSideOffset;

#[magnus::wrap(class = "Yass::Declarations::OutlineOffset")]
pub struct YOutlineOffset {
  border_side_offset: BorderSideOffset
}

impl YOutlineOffset {
  pub fn new(border_side_offset: BorderSideOffset) -> Self {
    Self { border_side_offset }
  }
}
