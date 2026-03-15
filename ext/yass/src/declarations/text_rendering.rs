use style::computed_values::text_rendering::T;

#[magnus::wrap(class = "Yass::Declarations::TextRendering")]
pub struct YTextRendering {
  specified_value: T
}

impl YTextRendering {
  pub fn new(specified_value: T) -> Self {
    Self { specified_value }
  }
}
