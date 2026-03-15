use style::values::specified::OverflowClipMargin;

#[magnus::wrap(class = "Yass::Declarations::OverflowClipMargin")]
pub struct YOverflowClipMargin {
  overflow_clip_margin: OverflowClipMargin
}

impl YOverflowClipMargin {
  pub fn new(overflow_clip_margin: OverflowClipMargin) -> Self {
    Self { overflow_clip_margin }
  }
}
