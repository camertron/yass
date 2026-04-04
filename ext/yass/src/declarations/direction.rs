use magnus::{Ruby, typed_data, value::Id};
use style::computed_values::direction::T;

#[magnus::wrap(class = "Yass::Declarations::Direction")]
pub struct YDirection {
    specified_value: T
}

impl YDirection {
    pub fn new(specified_value: T) -> Self {
        Self { specified_value }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.specified_value {
            T::Ltr => ruby.intern("ltr"),
            T::Rtl => ruby.intern("rtl"),
        }
    }
}
