use style::computed_values::text_decoration_style::T;
use magnus::{Ruby, typed_data, value::Id};

#[magnus::wrap(class = "Yass::Declarations::TextDecorationStyle")]
pub struct YTextDecorationStyle {
    specified_value: T
}

impl YTextDecorationStyle {
    pub fn new(specified_value: T) -> Self {
        Self { specified_value }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.specified_value {
            T::Solid => ruby.intern("solid"),
            T::Double => ruby.intern("double"),
            T::Dotted => ruby.intern("dotted"),
            T::Dashed => ruby.intern("dashed"),
            T::Wavy => ruby.intern("wavy"),
            T::MozNone => ruby.intern("moz_none"),
        }
    }
}
