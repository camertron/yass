use magnus::{DataTypeFunctions, Error, RArray, Ruby, TypedData, gc, typed_data};
use style::properties::longhands::background_image::{SingleSpecifiedValue, SpecifiedValue};

use crate::{cached_value_list::CachedValueList, declarations::images::image_to_value};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::BackgroundImage", mark)]
pub struct YBackgroundImage {
    values: CachedValueList<SingleSpecifiedValue>,
}

impl YBackgroundImage {
    pub fn new(specified_value: SpecifiedValue) -> Self {
        Self {
            values: CachedValueList::new(specified_value.0.to_vec(), |value, _ctx, ruby| {
                image_to_value(value, ruby)
            }),
        }
    }

    pub fn values(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        rb_self.values.to_a(ruby)
    }
}

impl DataTypeFunctions for YBackgroundImage {
    fn mark(&self, marker: &gc::Marker) {
        self.values.mark(marker);
    }
}
