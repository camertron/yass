use magnus::{DataTypeFunctions, Error, IntoValue, RArray, Ruby, gc::Marker, typed_data};
use style::values::specified::image::{ImageSet, ImageSetItem};

use crate::{cached_value_list::CachedValueList, declarations::images::image_set_item::YImageSetItem};

#[derive(magnus::TypedData)]
#[magnus(class = "Yass::Declarations::Image::ImageSet", mark)]
pub struct YImageSet {
    items: CachedValueList<ImageSetItem>,
}

impl YImageSet {
    pub fn new(image_set: ImageSet) -> Self {
        Self {
            items: CachedValueList::new(image_set.items.to_vec(), |item, _ctx, ruby| {
                YImageSetItem::new(item.clone()).into_value_with(ruby)
            }),
        }
    }

    pub fn items(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        rb_self.items.to_a(ruby)
    }
}

impl DataTypeFunctions for YImageSet {
    fn mark(&self, marker: &Marker) {
        self.items.mark(marker);
    }
}
