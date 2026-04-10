use magnus::{Ruby, typed_data, value::Id};
use style::values::computed::TextJustify;

#[magnus::wrap(class = "Yass::Declarations::TextJustify")]
pub struct YTextJustify {
    text_justify: TextJustify
}

impl YTextJustify {
    pub fn new(text_justify: TextJustify) -> Self {
        Self { text_justify }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.text_justify {
            TextJustify::Auto => ruby.intern("auto"),
            TextJustify::None => ruby.intern("none"),
            TextJustify::InterWord => ruby.intern("inter_word"),
            TextJustify::InterCharacter => ruby.intern("inter_character"),
        }
    }
}
