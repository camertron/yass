use magnus::{Ruby, typed_data, value::Id};
use style::values::specified::border::{BorderImageRepeat, BorderImageRepeatKeyword};

#[magnus::wrap(class = "Yass::Declarations::BorderImageRepeat")]
pub struct YBorderImageRepeat {
    border_image_repeat: BorderImageRepeat,
}

impl YBorderImageRepeat {
    pub fn new(border_image_repeat: BorderImageRepeat) -> Self {
        Self { border_image_repeat }
    }

    pub fn horizontal(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        border_image_repeat_keyword_to_id(rb_self.border_image_repeat.0, ruby)
    }

    pub fn vertical(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        border_image_repeat_keyword_to_id(rb_self.border_image_repeat.1, ruby)
    }
}

fn border_image_repeat_keyword_to_id(keyword: BorderImageRepeatKeyword, ruby: &Ruby) -> Id {
    match keyword {
        BorderImageRepeatKeyword::Stretch => ruby.intern("stretch"),
        BorderImageRepeatKeyword::Repeat => ruby.intern("repeat"),
        BorderImageRepeatKeyword::Round => ruby.intern("round"),
        BorderImageRepeatKeyword::Space => ruby.intern("space"),
    }
}
