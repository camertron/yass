use magnus::{Ruby, typed_data, value::Id};
use style::computed_values::object_fit::T;

#[magnus::wrap(class = "Yass::Declarations::ObjectFit")]
pub struct YObjectFit {
    object_fit: T
}

impl YObjectFit {
    pub fn new(object_fit: T) -> Self {
        Self { object_fit }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.object_fit {
            T::Fill => ruby.intern("fill"),
            T::Contain => ruby.intern("contain"),
            T::Cover => ruby.intern("cover"),
            T::ScaleDown => ruby.intern("scale_down"),
            T::None => ruby.intern("none"),
        }
    }
}
