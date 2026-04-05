use magnus::{Ruby, typed_data, value::Id};
use style::values::computed::Float;

#[magnus::wrap(class = "Yass::Declarations::Float")]
pub struct YFloat {
    float: Float
}

impl YFloat {
    pub fn new(float: Float) -> Self {
        Self { float }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.float {
            Float::Left => ruby.intern("left"),
            Float::Right => ruby.intern("right"),
            Float::None => ruby.intern("none"),
            Float::InlineStart => ruby.intern("inline_start"),
            Float::InlineEnd => ruby.intern("inline_end"),
        }
    }
}
