use magnus::{Ruby, typed_data, value::Id};
use style::values::computed::LineBreak;

#[magnus::wrap(class = "Yass::Declarations::LineBreak")]
pub struct YLineBreak {
    line_break: LineBreak
}

impl YLineBreak {
    pub fn new(line_break: LineBreak) -> Self {
        Self { line_break }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.line_break {
            LineBreak::Auto => ruby.intern("auto"),
            LineBreak::Loose => ruby.intern("loose"),
            LineBreak::Normal => ruby.intern("normal"),
            LineBreak::Strict => ruby.intern("strict"),
            LineBreak::Anywhere => ruby.intern("anywhere"),
        }
    }
}
