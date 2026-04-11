use magnus::{Ruby, typed_data, value::Id};
use style::computed_values::text_wrap_mode::T;

#[magnus::wrap(class = "Yass::Declarations::TextWrapMode")]
pub struct YTextWrapMode {
    specified_value: T
}

impl YTextWrapMode {
    pub fn new(specified_value: T) -> Self {
        Self { specified_value }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.specified_value {
            T::Wrap => ruby.intern("wrap"),
            T::Nowrap => ruby.intern("nowrap"),
        }
    }
}
