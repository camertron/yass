use magnus::{Ruby, typed_data, value::Id};
use style::values::computed::WordBreak;

#[magnus::wrap(class = "Yass::Declarations::WordBreak")]
pub struct YWordBreak {
    word_break: WordBreak
}

impl YWordBreak {
    pub fn new(word_break: WordBreak) -> Self {
        Self { word_break }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.word_break {
            WordBreak::Normal => ruby.intern("normal"),
            WordBreak::BreakAll => ruby.intern("break_all"),
            WordBreak::KeepAll => ruby.intern("keep_all"),
        }
    }
}
