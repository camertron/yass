use magnus::{DataTypeFunctions, Ruby, Value, gc::Marker, typed_data};
use style::values::{generics::color::GenericLightDark, specified::image::Image};

use crate::{cached_value::CachedValue, declarations::images::image_to_value};

#[derive(magnus::TypedData)]
#[magnus(class = "Yass::Declarations::Image::LightDark", mark)]
pub struct YImageLightDark {
    light: CachedValue<Image>,
    dark: CachedValue<Image>,
}

impl YImageLightDark {
    pub fn new(light_dark: GenericLightDark<Image>) -> Self {
        Self {
            light: CachedValue::new(light_dark.light, |image, ruby| {
                image_to_value(image, ruby)
            }),

            dark: CachedValue::new(light_dark.dark, |image, ruby| {
                image_to_value(image, ruby)
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

impl DataTypeFunctions for YImageLightDark {
    fn mark(&self, marker: &Marker) {
        self.light.mark(marker);
        self.dark.mark(marker);
    }
}
