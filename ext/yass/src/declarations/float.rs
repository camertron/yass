use style::values::computed::Float;

#[magnus::wrap(class = "Yass::Declarations::Float")]
pub struct YFloat {
  float: Float
}

impl YFloat {
  pub fn new(float: Float) -> Self {
    Self { float }
  }
}
