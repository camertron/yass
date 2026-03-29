use magnus::{DataTypeFunctions, IntoValue, Ruby, Value, gc::Marker, typed_data};
use style::values::specified::{Resolution, image::{Image, ImageSetItem}};

use crate::{cached_value::CachedValue, declarations::{images::image_to_value, resolution::YResolution}};

#[derive(magnus::TypedData)]
#[magnus(class = "Yass::Declarations::Image::ImageSetItem", mark)]
pub struct YImageSetItem {
    image: CachedValue<Image>,
    resolution: CachedValue<Resolution>,
    mime_type: String,
    has_mime_type: bool,
}

impl YImageSetItem {
    pub fn new(item: ImageSetItem) -> Self {
        Self {
            image: CachedValue::new(item.image, |image, ruby| {
                image_to_value(image, ruby)
            }),

            resolution: CachedValue::new(item.resolution, |resolution, ruby| {
                YResolution::new(resolution.clone()).into_value_with(ruby)
            }),

            mime_type: item.mime_type.to_string(),
            has_mime_type: item.has_mime_type,
        }
    }

    pub fn image(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.image.get(ruby)
    }

    pub fn resolution(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.resolution.get(ruby)
    }

    pub fn has_mime_type(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.has_mime_type
    }

    pub fn mime_type(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        ruby.str_new(&rb_self.mime_type).into_value_with(ruby)
    }
}

impl DataTypeFunctions for YImageSetItem {
    fn mark(&self, marker: &Marker) {
        self.image.mark(marker);
        self.resolution.mark(marker);
    }
}
