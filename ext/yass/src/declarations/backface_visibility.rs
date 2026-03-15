use style::computed_values::backface_visibility::T;

#[magnus::wrap(class = "Yass::Declarations::BackfaceVisibility")]
pub struct YBackfaceVisibility {
  specified_value: T
}

impl YBackfaceVisibility {
  pub fn new(specified_value: T) -> Self {
    Self { specified_value }
  }
}
