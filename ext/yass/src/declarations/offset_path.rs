use style::values::specified::OffsetPath;

#[magnus::wrap(class = "Yass::Declarations::OffsetPath")]
pub struct YOffsetPath {
  offset_path: OffsetPath
}

impl YOffsetPath {
  pub fn new(offset_path: OffsetPath) -> Self {
    Self { offset_path }
  }
}
