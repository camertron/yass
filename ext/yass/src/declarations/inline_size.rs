use magnus::{DataTypeFunctions, Ruby, TypedData, Value, gc, typed_data};
use style::values::specified::Size;

use crate::{cached_value::CachedValue, declarations::size::make_size};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::InlineSize", mark)]
pub struct YInlineSize {
    size: CachedValue<Size>
}

impl YInlineSize {
    pub fn new(size: Size) -> Self {
        Self {
            size: CachedValue::new(size, |s, ruby| {
                make_size(s, ruby)
            })
        }
    }

    pub fn size(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.size.get(ruby)
    }
}

impl DataTypeFunctions for YInlineSize {
    fn mark(&self, marker: &gc::Marker) {
        self.size.mark(marker);
    }
}
