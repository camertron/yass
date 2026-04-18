use magnus::{DataTypeFunctions, Ruby, TypedData, Value, gc, typed_data};
use style::values::specified::Inset;

use crate::{cached_value::CachedValue, declarations::inset::make_inset};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::InsetInlineEnd", mark)]
pub struct YInsetInlineEnd {
    inset: CachedValue<Inset>,
}

impl YInsetInlineEnd {
    pub fn new(inset: Inset) -> Self {
        Self {
            inset: CachedValue::new(inset, |value, ruby| make_inset(value, ruby)),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.inset.get(ruby)
    }
}

impl DataTypeFunctions for YInsetInlineEnd {
    fn mark(&self, marker: &gc::Marker) {
        self.inset.mark(marker);
    }
}
