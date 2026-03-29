use magnus::{DataTypeFunctions, IntoValue, Ruby, Value, gc::Marker, typed_data};
use style::values::specified::{Percentage, image::{CrossFadeElement, CrossFadeImage}};

use crate::{cached_value::CachedValue, declarations::{images::cross_fade_image::make_cross_fade_image, percentage::YPercentage}};

#[derive(magnus::TypedData)]
#[magnus(class = "Yass::Declarations::Image::CrossFadeElement", mark)]
pub struct YCrossFadeElement {
    percent: CachedValue<Option<Percentage>>,
    image: CachedValue<CrossFadeImage>,
}

impl YCrossFadeElement {
    pub fn new(element: CrossFadeElement) -> Self {
        Self {
            percent: CachedValue::new(element.percent.into_rust(), |percent, ruby| match percent {
                Some(percent) => {
                    YPercentage::new(percent.get()).into_value_with(ruby)
                },

                None => ruby.qnil().into_value_with(ruby),
            }),

            image: CachedValue::new(element.image, |image, ruby| {
                make_cross_fade_image(image, ruby).into_value_with(ruby)
            }),
        }
    }

    pub fn percent(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.percent.get(ruby)
    }

    pub fn image(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.image.get(ruby)
    }
}

impl DataTypeFunctions for YCrossFadeElement {
    fn mark(&self, marker: &Marker) {
        self.percent.mark(marker);
        self.image.mark(marker);
    }
}
