use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc, typed_data};
use style::values::{generics::NonNegative, specified::{NumberOrPercentage, Zoom}};

use crate::{cached_value::CachedValue, declarations::{number::YNumber, percentage::YPercentage}};

fn make_zoom_value(value: &NonNegative<NumberOrPercentage>, ruby: &Ruby) -> Value {
    match value.0 {
        NumberOrPercentage::Number(number) => YNumber::new(number.get()).into_value_with(ruby),
        NumberOrPercentage::Percentage(percentage) => {
            YPercentage::new(percentage.get()).into_value_with(ruby)
        }
    }
}

fn make_zoom(zoom: &Zoom, ruby: &Ruby) -> Value {
    match zoom {
        Zoom::Normal => YZoomNormal::new().into_value_with(ruby),
        Zoom::Document => YZoomDocument::new().into_value_with(ruby),
        Zoom::Value(value) => YZoomValue::new(*value).into_value_with(ruby),
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Zoom", mark)]
pub struct YZoom {
    zoom: CachedValue<Zoom>,
}

impl YZoom {
    pub fn new(zoom: Zoom) -> Self {
        Self {
            zoom: CachedValue::new(zoom, make_zoom),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.zoom.get(ruby)
    }
}

impl DataTypeFunctions for YZoom {
    fn mark(&self, marker: &gc::Marker) {
        self.zoom.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::Declarations::Zoom::Normal")]
pub struct YZoomNormal {}

impl YZoomNormal {
    pub fn new() -> Self {
        Self {}
    }
}

#[magnus::wrap(class = "Yass::Declarations::Zoom::Document")]
pub struct YZoomDocument {}

impl YZoomDocument {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Zoom::Value", mark)]
pub struct YZoomValue {
    value: CachedValue<NonNegative<NumberOrPercentage>>,
}

impl YZoomValue {
    pub fn new(value: NonNegative<NumberOrPercentage>) -> Self {
        Self {
            value: CachedValue::new(value, make_zoom_value),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.value.get(ruby)
    }
}

impl DataTypeFunctions for YZoomValue {
    fn mark(&self, marker: &gc::Marker) {
        self.value.mark(marker);
    }
}
