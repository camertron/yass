use magnus::{Ruby, typed_data, value::Id};
use style::values::computed::TextAlignLast;

#[magnus::wrap(class = "Yass::Declarations::TextAlignLast")]
pub struct YTextAlignLast {
    text_align_last: TextAlignLast
}

impl YTextAlignLast {
    pub fn new(text_align_last: TextAlignLast) -> Self {
        Self { text_align_last }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.text_align_last {
            TextAlignLast::Auto => ruby.intern("auto"),
            TextAlignLast::Start => ruby.intern("start"),
            TextAlignLast::End => ruby.intern("end"),
            TextAlignLast::Left => ruby.intern("left"),
            TextAlignLast::Right => ruby.intern("right"),
            TextAlignLast::Center => ruby.intern("center"),
            TextAlignLast::Justify => ruby.intern("justify"),
        }
    }
}
