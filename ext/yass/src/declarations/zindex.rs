use style::values::specified::ZIndex;

#[magnus::wrap(class = "Yass::Declarations::ZIndex")]
pub struct YZIndex {
  zindex: ZIndex
}

impl YZIndex {
  pub fn new(zindex: ZIndex) -> Self {
    Self { zindex }
  }
}
