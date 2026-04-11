use magnus::{Ruby, typed_data, value::Id};
use style::computed_values::unicode_bidi::T;

#[magnus::wrap(class = "Yass::Declarations::UnicodeBidi")]
pub struct YUnicodeBidi {
    specified_value: T
}

impl YUnicodeBidi {
    pub fn new(specified_value: T) -> Self {
        Self { specified_value }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.specified_value {
            T::Normal => ruby.intern("normal"),
            T::Embed => ruby.intern("embed"),
            T::Isolate => ruby.intern("isolate"),
            T::BidiOverride => ruby.intern("bidi_override"),
            T::IsolateOverride => ruby.intern("isolate_override"),
            T::Plaintext => ruby.intern("plaintext"),
        }
    }
}
