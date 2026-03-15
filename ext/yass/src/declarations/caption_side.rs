use style::values::specified::table::CaptionSide;

#[magnus::wrap(class = "Yass::Declarations::CaptionSide")]
pub struct YCaptionSide {
  caption_side: CaptionSide
}

impl YCaptionSide {
  pub fn new(caption_side: CaptionSide) -> Self {
    Self { caption_side }
  }
}
