use magnus::{DataTypeFunctions, Error, IntoValue, RArray, Ruby, TypedData, Value, gc, typed_data};
use style::{
    properties::longhands::box_shadow::{SingleSpecifiedValue, SpecifiedValue},
    values::specified::{Color, Length, NonNegativeLength},
};

use crate::{
    cached_value::CachedValue,
    cached_value_list::CachedValueList,
    declarations::{color::color::make_color, length::length_to_value},
};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::BoxShadow", mark)]
pub struct YBoxShadow {
    values: CachedValueList<SingleSpecifiedValue>,
}

impl YBoxShadow {
    pub fn new(specified_value: SpecifiedValue) -> Self {
        Self {
            values: CachedValueList::new(specified_value.0.to_vec(), |value, _ctx, ruby| {
                YBoxShadowShadow::new(value.clone()).into_value_with(ruby)
            }),
        }
    }

    pub fn values(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        rb_self.values.to_a(ruby)
    }
}

impl DataTypeFunctions for YBoxShadow {
    fn mark(&self, marker: &gc::Marker) {
        self.values.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::BoxShadow::Shadow", mark)]
pub struct YBoxShadowShadow {
    color: CachedValue<Option<Color>>,
    horizontal: CachedValue<Length>,
    vertical: CachedValue<Length>,
    blur: CachedValue<Option<NonNegativeLength>>,
    spread: CachedValue<Option<Length>>,
    inset: bool,
}

impl YBoxShadowShadow {
    pub fn new(value: SingleSpecifiedValue) -> Self {
        Self {
            color: CachedValue::new(value.base.color, |color, ruby| match color {
                Some(color) => make_color(color, ruby),
                None => ruby.qnil().into_value_with(ruby),
            }),

            horizontal: CachedValue::new(value.base.horizontal, |length, ruby| {
                length_to_value(length, ruby)
            }),

            vertical: CachedValue::new(value.base.vertical, |length, ruby| {
                length_to_value(length, ruby)
            }),

            blur: CachedValue::new(value.base.blur, |blur, ruby| match blur {
                Some(blur) => length_to_value(&blur.0, ruby),
                None => ruby.qnil().into_value_with(ruby),
            }),

            spread: CachedValue::new(value.spread, |spread, ruby| match spread {
                Some(spread) => length_to_value(spread, ruby),
                None => ruby.qnil().into_value_with(ruby),
            }),

            inset: value.inset,
        }
    }

    pub fn color(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.color.get(ruby)
    }

    pub fn horizontal(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.horizontal.get(ruby)
    }

    pub fn vertical(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.vertical.get(ruby)
    }

    pub fn blur(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.blur.get(ruby)
    }

    pub fn spread(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.spread.get(ruby)
    }

    pub fn is_inset(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.inset
    }
}

impl DataTypeFunctions for YBoxShadowShadow {
    fn mark(&self, marker: &gc::Marker) {
        self.color.mark(marker);
        self.horizontal.mark(marker);
        self.vertical.mark(marker);
        self.blur.mark(marker);
        self.spread.mark(marker);
    }
}
