use magnus::{DataTypeFunctions, Error, RArray, Ruby, TypedData, gc, typed_data};
use style::{properties::longhands::backdrop_filter::SpecifiedValue, values::specified::effects::Filter as SpecifiedFilter};

use crate::{cached_value_list::CachedValueList, declarations::filter::filter_to_value};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::BackdropFilter", mark)]
pub struct YBackdropFilter {
    values: CachedValueList<SpecifiedFilter>,
}

impl YBackdropFilter {
    pub fn new(specified_value: SpecifiedValue) -> Self {
        Self {
            values: CachedValueList::new(specified_value.0.to_vec(), |value, _ctx, ruby| {
                filter_to_value(value, ruby)
            }),
        }
    }

    pub fn values(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        rb_self.values.to_a(ruby)
    }
}

impl DataTypeFunctions for YBackdropFilter {
    fn mark(&self, marker: &gc::Marker) {
        self.values.mark(marker);
    }
}
