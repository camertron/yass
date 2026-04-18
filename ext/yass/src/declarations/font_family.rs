use magnus::{DataTypeFunctions, Ruby, TypedData, Value, gc, typed_data};
use style::values::specified::FontFamily;

use crate::{cached_value::CachedValue, rules::fonts::family::make_font_family};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::FontFamily", mark)]
pub struct YFontFamily {
    font_family: CachedValue<FontFamily>
}

impl YFontFamily {
    pub fn new(font_family: FontFamily) -> Self {
        Self {
            font_family: CachedValue::new(font_family, make_font_family),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.font_family.get(ruby)
    }
}

impl DataTypeFunctions for YFontFamily {
    fn mark(&self, marker: &gc::Marker) {
        self.font_family.mark(marker);
    }
}
