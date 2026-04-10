use magnus::{Ruby, typed_data, value::Id};
use style::values::specified::{TextAlign, TextAlignKeyword};

#[magnus::wrap(class = "Yass::Declarations::TextAlign")]
pub struct YTextAlign {
    text_align: TextAlign
}

impl YTextAlign {
    pub fn new(text_align: TextAlign) -> Self {
        Self { text_align }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.text_align {
            TextAlign::Keyword(keyword) => match keyword {
                TextAlignKeyword::Start => ruby.intern("start"),
                TextAlignKeyword::Left => ruby.intern("left"),
                TextAlignKeyword::Right => ruby.intern("right"),
                TextAlignKeyword::Center => ruby.intern("center"),
                TextAlignKeyword::Justify => ruby.intern("justify"),
                TextAlignKeyword::End => ruby.intern("end"),
                TextAlignKeyword::MozCenter => ruby.intern("moz_center"),
                TextAlignKeyword::MozLeft => ruby.intern("moz_left"),
                TextAlignKeyword::MozRight => ruby.intern("moz_right"),
            },

            TextAlign::MozCenterOrInherit => ruby.intern("moz_center_or_inherit"),
        }
    }
}
