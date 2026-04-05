use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc, typed_data};
use style::values::specified::{LengthPercentage, TrackBreadth};

use crate::{
    cached_value::CachedValue,
    declarations::size::length_percentage_to_value,
};

pub fn make_track_breadth_value(track_breadth: &TrackBreadth, ruby: &Ruby) -> Value {
    match track_breadth {
        TrackBreadth::Breadth(length_percentage) => {
            YTrackBreadthLengthPercentage::new(length_percentage.clone())
                .into_value_with(ruby)
        }
        TrackBreadth::Fr(fr) => YTrackBreadthFr::new(*fr).into_value_with(ruby),
        TrackBreadth::Auto => YTrackBreadthAuto::new().into_value_with(ruby),
        TrackBreadth::MinContent => YTrackBreadthMinContent::new().into_value_with(ruby),
        TrackBreadth::MaxContent => YTrackBreadthMaxContent::new().into_value_with(ruby),
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::TrackBreadth::LengthPercentage", mark)]
pub struct YTrackBreadthLengthPercentage {
    value: CachedValue<LengthPercentage>,
}

impl YTrackBreadthLengthPercentage {
    pub fn new(length_percentage: LengthPercentage) -> Self {
        Self {
            value: CachedValue::new(length_percentage, |value, ruby| {
                length_percentage_to_value(value.clone(), ruby)
            }),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.value.get(ruby)
    }
}

impl DataTypeFunctions for YTrackBreadthLengthPercentage {
    fn mark(&self, marker: &gc::Marker) {
        self.value.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::Declarations::TrackBreadth::Fr")]
pub struct YTrackBreadthFr {
    value: f32,
}

impl YTrackBreadthFr {
    pub fn new(value: f32) -> Self {
        Self { value }
    }

    pub fn value(_ruby: &Ruby, rb_self: &Self) -> f32 {
        rb_self.value
    }
}

#[magnus::wrap(class = "Yass::Declarations::TrackBreadth::Auto")]
pub struct YTrackBreadthAuto {}

impl YTrackBreadthAuto {
    pub fn new() -> Self {
        Self {}
    }
}

#[magnus::wrap(class = "Yass::Declarations::TrackBreadth::MinContent")]
pub struct YTrackBreadthMinContent {}

impl YTrackBreadthMinContent {
    pub fn new() -> Self {
        Self {}
    }
}

#[magnus::wrap(class = "Yass::Declarations::TrackBreadth::MaxContent")]
pub struct YTrackBreadthMaxContent {}

impl YTrackBreadthMaxContent {
    pub fn new() -> Self {
        Self {}
    }
}
