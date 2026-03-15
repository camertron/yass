use style::values::specified::Zoom;

#[magnus::wrap(class = "Yass::Declarations::Zoom")]
pub struct YZoom {
  zoom: Zoom
}

impl YZoom {
  pub fn new(zoom: Zoom) -> Self {
    Self { zoom }
  }
}
