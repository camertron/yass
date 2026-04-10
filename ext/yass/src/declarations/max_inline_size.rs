use magnus::{DataTypeFunctions, Ruby, TypedData, Value, gc, typed_data};
use style::values::specified::MaxSize;

use crate::{cached_value::CachedValue, declarations::size::make_max_size};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::MaxInlineSize", mark)]
pub struct YMaxInlineSize {
    max_size: CachedValue<MaxSize>
}

impl YMaxInlineSize {
    pub fn new(max_size: MaxSize) -> Self {
        Self {
            max_size: CachedValue::new(max_size, |ms, ruby| {
                make_max_size(ms, ruby)
            })
        }
    }

    pub fn size(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.max_size.get(ruby)
    }
}

impl DataTypeFunctions for YMaxInlineSize {
    fn mark(&self, marker: &gc::Marker) {
        self.max_size.mark(marker);
    }
}
