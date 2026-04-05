use magnus::{Ruby, typed_data, value::Id};
use style::computed_values::flex_wrap::T;

#[magnus::wrap(class = "Yass::Declarations::FlexWrap")]
pub struct YFlexWrap {
    specified_value: T
}

impl YFlexWrap {
    pub fn new(specified_value: T) -> Self {
        Self { specified_value }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.specified_value {
            T::Nowrap => ruby.intern("nowrap"),
            T::Wrap => ruby.intern("wrap"),
            T::WrapReverse => ruby.intern("wrap_reverse"),
        }
    }
}
