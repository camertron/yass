use magnus::{Ruby, typed_data, value::Id};
use style::computed_values::backface_visibility::T;

#[magnus::wrap(class = "Yass::Declarations::BackfaceVisibility")]
pub struct YBackfaceVisibility {
    specified_value: T
}

impl YBackfaceVisibility {
    pub fn new(specified_value: T) -> Self {
        Self { specified_value }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.specified_value {
            T::Visible => ruby.intern("visible"),
            T::Hidden => ruby.intern("hidden"),
        }
    }
}
