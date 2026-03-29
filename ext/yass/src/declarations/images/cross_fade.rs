use magnus::{DataTypeFunctions, Error, IntoValue, RArray, Ruby, TypedData, gc::Marker, typed_data};
use style::values::specified::image::{CrossFade, CrossFadeElement};

use crate::{cached_value_list::CachedValueList, declarations::images::cross_fade_element::YCrossFadeElement};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Image::CrossFade", mark)]
pub struct YCrossFade {
    elements: CachedValueList<CrossFadeElement>,
}

impl YCrossFade {
    pub fn new(cross_fade: CrossFade) -> Self {
        Self {
            elements: CachedValueList::new(
                cross_fade.elements.to_vec(),
                |element, _ctx, ruby| {
                    YCrossFadeElement::new(element.clone()).into_value_with(ruby)
                },
            ),
        }
    }

    pub fn elements(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        rb_self.elements.to_a(ruby)
    }
}

impl DataTypeFunctions for YCrossFade {
    fn mark(&self, marker: &Marker) {
        self.elements.mark(marker);
    }
}
