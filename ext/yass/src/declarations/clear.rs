use magnus::{Ruby, typed_data, value::Id};
use style::values::computed::Clear;

#[magnus::wrap(class = "Yass::Declarations::Clear")]
pub struct YClear {
    clear: Clear
}

impl YClear {
    pub fn new(clear: Clear) -> Self {
        Self { clear }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.clear {
            Clear::None => ruby.intern("none"),
            Clear::Left => ruby.intern("left"),
            Clear::Right => ruby.intern("right"),
            Clear::Both => ruby.intern("both"),
            Clear::InlineStart => ruby.intern("inline_start"),
            Clear::InlineEnd => ruby.intern("inline_end"),
        }
    }
}
