use magnus::{DataTypeFunctions, Ruby, TypedData, Value, gc, typed_data};
use style::values::specified::Margin;

use crate::{cached_value::CachedValue, declarations::margin::make_margin};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::MarginBlockStart", mark)]
pub struct YMarginBlockStart {
    margin: CachedValue<Margin>,
}

impl YMarginBlockStart {
    pub fn new(margin: Margin) -> Self {
        Self {
            margin: CachedValue::new(margin, |value, ruby| make_margin(value, ruby)),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.margin.get(ruby)
    }
}

impl DataTypeFunctions for YMarginBlockStart {
    fn mark(&self, marker: &gc::Marker) {
        self.margin.mark(marker);
    }
}
