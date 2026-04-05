use magnus::{DataTypeFunctions, Error, RArray, Ruby, TypedData, gc, typed_data};
use style::values::specified::{ImplicitGridTracks, TrackSize};

use crate::{
    cached_value_list::CachedValueList, declarations::track_size::make_track_size_value,
};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::GridAutoColumns", mark)]
pub struct YGridAutoColumns {
    values: CachedValueList<TrackSize>,
}

impl YGridAutoColumns {
    pub fn new(implicit_grid_tracks: ImplicitGridTracks) -> Self {
        Self {
            values: CachedValueList::new(implicit_grid_tracks.0.to_vec(), |value, _ctx, ruby| {
                make_track_size_value(value, ruby)
            }),
        }
    }

    pub fn values(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        rb_self.values.to_a(ruby)
    }
}

impl DataTypeFunctions for YGridAutoColumns {
    fn mark(&self, marker: &gc::Marker) {
        self.values.mark(marker);
    }
}
