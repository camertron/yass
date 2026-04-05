use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc, typed_data};
use style::values::{specified::TrackSize, specified::TrackBreadth};

use crate::{cached_value::CachedValue, declarations::track_breadth::make_track_breadth_value};

pub fn make_track_size_value(track_size: &TrackSize, ruby: &Ruby) -> Value {
    match track_size {
        TrackSize::Breadth(breadth) => make_track_breadth_value(breadth, ruby),
        TrackSize::Minmax(min, max) => {
            YTrackSizeMinmax::new(min.clone(), max.clone()).into_value_with(ruby)
        }
        TrackSize::FitContent(breadth) => {
            YTrackSizeFitContent::new(breadth.clone()).into_value_with(ruby)
        }
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::TrackSize::Minmax", mark)]
pub struct YTrackSizeMinmax {
    min: CachedValue<TrackBreadth>,
    max: CachedValue<TrackBreadth>,
}

impl YTrackSizeMinmax {
    pub fn new(min: TrackBreadth, max: TrackBreadth) -> Self {
        Self {
            min: CachedValue::new(min, make_track_breadth_value),
            max: CachedValue::new(max, make_track_breadth_value),
        }
    }

    pub fn min(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.min.get(ruby)
    }

    pub fn max(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.max.get(ruby)
    }
}

impl DataTypeFunctions for YTrackSizeMinmax {
    fn mark(&self, marker: &gc::Marker) {
        self.min.mark(marker);
        self.max.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::TrackSize::FitContent", mark)]
pub struct YTrackSizeFitContent {
    value: CachedValue<TrackBreadth>,
}

impl YTrackSizeFitContent {
    pub fn new(track_breadth: TrackBreadth) -> Self {
        Self {
            value: CachedValue::new(track_breadth, make_track_breadth_value),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.value.get(ruby)
    }
}

impl DataTypeFunctions for YTrackSizeFitContent {
    fn mark(&self, marker: &gc::Marker) {
        self.value.mark(marker);
    }
}
