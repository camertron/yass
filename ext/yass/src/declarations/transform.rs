use style::values::specified::Transform;

#[magnus::wrap(class = "Yass::Declarations::Transform")]
pub struct YTransform {
  transform: Transform
}

impl YTransform {
  pub fn new(transform: Transform) -> Self {
    Self { transform }
  }
}
