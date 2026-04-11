use magnus::{Ruby, typed_data, value::Id};
use style::computed_values::_webkit_text_security::T;

#[magnus::wrap(class = "Yass::Declarations::WebkitTextSecurity")]
pub struct YWebkitTextSecurity {
    specified_value: T
}

impl YWebkitTextSecurity {
    pub fn new(specified_value: T) -> Self {
        Self { specified_value }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.specified_value {
            T::None => ruby.intern("none"),
            T::Circle => ruby.intern("circle"),
            T::Disc => ruby.intern("disc"),
            T::Square => ruby.intern("square"),
        }
    }
}
