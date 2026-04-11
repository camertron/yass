use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc, typed_data};
use style::values::specified::{Integer, ZIndex};

use crate::cached_value::CachedValue;

fn make_zindex_value(zindex: &ZIndex, ruby: &Ruby) -> Value {
    match zindex {
        ZIndex::Integer(integer) => YZIndexInteger::new(integer.clone()).into_value_with(ruby),
        ZIndex::Auto => YZIndexAuto::new().into_value_with(ruby),
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::ZIndex", mark)]
pub struct YZIndex {
    value: CachedValue<ZIndex>,
}

impl YZIndex {
    pub fn new(zindex: ZIndex) -> Self {
        Self {
            value: CachedValue::new(zindex, make_zindex_value),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.value.get(ruby)
    }
}

impl DataTypeFunctions for YZIndex {
    fn mark(&self, marker: &gc::Marker) {
        self.value.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::Declarations::ZIndex::Auto")]
pub struct YZIndexAuto {}

impl YZIndexAuto {
    pub fn new() -> Self {
        Self {}
    }
}

#[magnus::wrap(class = "Yass::Declarations::ZIndex::Integer")]
pub struct YZIndexInteger {
    value: i32,
}

impl YZIndexInteger {
    pub fn new(integer: Integer) -> Self {
        Self {
            value: integer.value(),
        }
    }

    pub fn value(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> i32 {
        rb_self.value
    }
}
