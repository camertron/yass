use magnus::{DataTypeFunctions, Ruby, TypedData, Value, gc, typed_data};
use style::values::{
    generics::color::GenericLightDark,
    specified::Color,
};

use crate::{cached_value::CachedValue, declarations::color::color::make_color};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Color::LightDark", mark)]
pub struct YLightDark {
    light: CachedValue<Color>,
    dark: CachedValue<Color>,
}

impl YLightDark {
    pub fn new(light_dark: GenericLightDark<Color>) -> Self {
        Self {
            light: CachedValue::new(light_dark.light, |color, ruby| {
                make_color(&color, ruby)
            }),

            dark: CachedValue::new(light_dark.dark, |color, ruby| {
                make_color(&color, ruby)
            }),
        }
    }

    pub fn light(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.light.get(ruby)
    }

    pub fn dark(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.dark.get(ruby)
    }
}

impl DataTypeFunctions for YLightDark {
    fn mark(&self, marker: &gc::Marker) {
        self.light.mark(marker);
        self.dark.mark(marker);
    }
}
