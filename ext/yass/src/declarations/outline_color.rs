use magnus::{DataTypeFunctions, Ruby, TypedData, Value, gc, typed_data};
use style::values::specified::Color;

use crate::{cached_value::CachedValue, declarations::color::color::make_color};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::OutlineColor", mark)]
pub struct YOutlineColor {
    color: CachedValue<Color>
}

impl YOutlineColor {
    pub fn new(color: Color) -> Self {
        Self {
            color: CachedValue::new(color, |color, ruby| {
                make_color(color, ruby)
            })
        }
    }

    pub fn color(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.color.get(ruby)
    }
}

impl DataTypeFunctions for YOutlineColor {
    fn mark(&self, marker: &gc::Marker) {
        self.color.mark(marker);
    }
}
