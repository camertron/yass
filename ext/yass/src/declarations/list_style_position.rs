use magnus::{Ruby, typed_data, value::Id};
use style::computed_values::list_style_position::T;

#[magnus::wrap(class = "Yass::Declarations::ListStylePosition")]
pub struct YListStylePosition {
    specified_value: T
}

impl YListStylePosition {
    pub fn new(specified_value: T) -> Self {
        Self { specified_value }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.specified_value {
            T::Inside => ruby.intern("inside"),
            T::Outside => ruby.intern("outside"),
        }
    }
}
