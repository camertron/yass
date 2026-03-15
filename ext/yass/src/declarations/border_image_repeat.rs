use style::values::computed::BorderImageRepeat;

#[magnus::wrap(class = "Yass::Declarations::BorderImageRepeat")]
pub struct YBorderImageRepeat {
  border_image_repeat: BorderImageRepeat
}

impl YBorderImageRepeat {
  pub fn new(border_image_repeat: BorderImageRepeat) -> Self {
    Self { border_image_repeat }
  }
}
