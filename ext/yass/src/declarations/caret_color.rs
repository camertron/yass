use magnus::{DataTypeFunctions, Ruby, TypedData, Value, gc, typed_data};
use style::values::generics::color::GenericCaretColor;
use style::values::specified::color::Color;

use crate::declarations::color::color::make_color_or_auto;
use crate::{cached_value::CachedValue};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::CaretColor", mark)]
pub struct YCaretColor {
    color: CachedValue<GenericCaretColor<Color>>,
}

impl YCaretColor {
    pub fn new(caret_color: GenericCaretColor<Color>) -> Self {
        Self {
            color: CachedValue::new(caret_color, |caret_color, ruby| {
                make_color_or_auto(&caret_color.0, ruby)
            }),
        }
    }

    pub fn color(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.color.get(ruby)
    }
}

impl DataTypeFunctions for YCaretColor {
    fn mark(&self, marker: &gc::Marker) {
        self.color.mark(marker);
    }
}
