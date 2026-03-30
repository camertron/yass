use magnus::{DataTypeFunctions, Error, IntoValue, RArray, Ruby, TypedData, Value, gc, typed_data};
use style::{
    properties::longhands::background_size::{SingleSpecifiedValue, SpecifiedValue},
    values::specified::{background::BackgroundSize, length::NonNegativeLengthPercentageOrAuto},
};

use crate::{cached_value::CachedValue, cached_value_list::CachedValueList, declarations::size::length_percentage_to_value};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::BackgroundSize", mark)]
pub struct YBackgroundSize {
    values: CachedValueList<SingleSpecifiedValue>,
}

impl YBackgroundSize {
    pub fn new(specified_value: SpecifiedValue) -> Self {
        Self {
            values: CachedValueList::new(specified_value.0.to_vec(), |value, _ctx, ruby| {
                make_background_size(value, ruby)
            }),
        }
    }

    pub fn values(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        rb_self.values.to_a(ruby)
    }
}

impl DataTypeFunctions for YBackgroundSize {
    fn mark(&self, marker: &gc::Marker) {
        self.values.mark(marker);
    }
}

pub fn make_background_size(value: &SingleSpecifiedValue, ruby: &Ruby) -> Value {
    match value {
        BackgroundSize::ExplicitSize { width, height } => {
            YBackgroundSizeExplicitSize::new(width.clone(), height.clone()).into_value_with(ruby)
        }
        BackgroundSize::Cover => YBackgroundSizeCover::new().into_value_with(ruby),
        BackgroundSize::Contain => YBackgroundSizeContain::new().into_value_with(ruby),
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::BackgroundSize::ExplicitSize", mark)]
pub struct YBackgroundSizeExplicitSize {
    width: CachedValue<NonNegativeLengthPercentageOrAuto>,
    height: CachedValue<NonNegativeLengthPercentageOrAuto>,
}

impl YBackgroundSizeExplicitSize {
    pub fn new(
        width: NonNegativeLengthPercentageOrAuto,
        height: NonNegativeLengthPercentageOrAuto,
    ) -> Self {
        Self {
            width: CachedValue::new(width, |width, ruby| {
                length_percentage_or_auto_to_value(width, ruby)
            }),
            height: CachedValue::new(height, |height, ruby| {
                length_percentage_or_auto_to_value(height, ruby)
            }),
        }
    }

    pub fn width(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.width.get(ruby)
    }

    pub fn height(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.height.get(ruby)
    }
}

impl DataTypeFunctions for YBackgroundSizeExplicitSize {
    fn mark(&self, marker: &gc::Marker) {
        self.width.mark(marker);
        self.height.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::Declarations::BackgroundSize::Cover")]
pub struct YBackgroundSizeCover {}

impl YBackgroundSizeCover {
    pub fn new() -> Self {
        Self {}
    }
}

#[magnus::wrap(class = "Yass::Declarations::BackgroundSize::Contain")]
pub struct YBackgroundSizeContain {}

impl YBackgroundSizeContain {
    pub fn new() -> Self {
        Self {}
    }
}

#[magnus::wrap(class = "Yass::Declarations::BackgroundSize::Auto")]
pub struct YBackgroundSizeAuto {}

impl YBackgroundSizeAuto {
    pub fn new() -> Self {
        Self {}
    }
}

fn length_percentage_or_auto_to_value(
    value: &NonNegativeLengthPercentageOrAuto,
    ruby: &Ruby,
) -> Value {
    match value {
        NonNegativeLengthPercentageOrAuto::Auto => YBackgroundSizeAuto::new().into_value_with(ruby),
        NonNegativeLengthPercentageOrAuto::LengthPercentage(length_percentage) => {
            length_percentage_to_value(length_percentage.0.clone(), ruby)
        }
    }
}
