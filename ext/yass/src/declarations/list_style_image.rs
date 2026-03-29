use magnus::{DataTypeFunctions, Ruby, TypedData, Value, gc, typed_data};
use style::values::specified::Image;

use crate::{cached_value::CachedValue, declarations::images::image_to_value};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::ListStyleImage", mark)]
pub struct YListStyleImage {
    image: CachedValue<Image>,
}

impl YListStyleImage {
    pub fn new(image: Image) -> Self {
        Self {
            image: CachedValue::new(image, |image, ruby| image_to_value(image, ruby)),
        }
    }

    pub fn image(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.image.get(ruby)
    }
}

impl DataTypeFunctions for YListStyleImage {
    fn mark(&self, marker: &gc::Marker) {
        self.image.mark(marker);
    }
}
