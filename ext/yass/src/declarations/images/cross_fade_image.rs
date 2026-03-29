use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc::Marker, typed_data};
use style::values::specified::{Color, Image, image::CrossFadeImage};

use crate::{cached_value::CachedValue, declarations::{color::color::make_color, images::image_to_value}};

pub fn make_cross_fade_image(image: &CrossFadeImage, ruby: &Ruby) -> Value {
    match image {
        CrossFadeImage::Image(image) => {
            YCrossFadeImage::new(image.clone()).into_value_with(ruby)
        },

        CrossFadeImage::Color(color) => {
            YCrossFadeColor::new(color.clone()).into_value_with(ruby)
        },
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Image::CrossFadeImage", mark)]
pub struct YCrossFadeImage {
    image: CachedValue<Image>,
}

impl YCrossFadeImage {
    pub fn new(image: Image) -> Self {
        Self {
            image: CachedValue::new(image, |image, ruby| {
                image_to_value(image, ruby)
            })
        }
    }

    pub fn image(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.image.get(ruby)
    }
}

impl DataTypeFunctions for YCrossFadeImage {
    fn mark(&self, marker: &Marker) {
        self.image.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Image::CrossFadeColor", mark)]
pub struct YCrossFadeColor {
    color: CachedValue<Color>,
}

impl YCrossFadeColor {
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

impl DataTypeFunctions for YCrossFadeColor {
    fn mark(&self, marker: &Marker) {
        self.color.mark(marker);
    }
}
