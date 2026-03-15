use style::values::computed::ImageRendering;

#[magnus::wrap(class = "Yass::Declarations::ImageRendering")]
pub struct YImageRendering {
  image_rendering: ImageRendering
}

impl YImageRendering {
  pub fn new(image_rendering: ImageRendering) -> Self {
    Self { image_rendering }
  }
}
