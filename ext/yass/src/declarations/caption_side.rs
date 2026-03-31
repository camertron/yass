use magnus::{Ruby, typed_data, value::Id};
use style::values::specified::table::CaptionSide;

#[magnus::wrap(class = "Yass::Declarations::CaptionSide")]
pub struct YCaptionSide {
    caption_side: CaptionSide
}

impl YCaptionSide {
    pub fn new(caption_side: CaptionSide) -> Self {
        Self { caption_side }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.caption_side {
            CaptionSide::Top => ruby.intern("top"),
            CaptionSide::Bottom => ruby.intern("bottom"),
        }
    }
}
