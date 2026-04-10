use magnus::{DataTypeFunctions, Error, IntoValue, RArray, Ruby, TypedData, Value, gc, typed_data};
use style::{
    properties::longhands::text_shadow::{SingleSpecifiedValue, SpecifiedValue},
    values::{
        generics::NonNegative,
        specified::{Color, Length},
    },
};

use crate::{
    cached_value::CachedValue,
    cached_value_list::CachedValueList,
    declarations::{color::color::make_color, length::length_to_value},
};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::TextShadow", mark)]
pub struct YTextShadow {
    values: CachedValueList<SingleSpecifiedValue>,
}

impl YTextShadow {
    pub fn new(specified_value: SpecifiedValue) -> Self {
        Self {
            values: CachedValueList::new(specified_value.0.to_vec(), |value, _ctx, ruby| {
                YTextShadowShadow::new(value.clone()).into_value_with(ruby)
            }),
        }
    }

    pub fn values(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        rb_self.values.to_a(ruby)
    }
}

impl DataTypeFunctions for YTextShadow {
    fn mark(&self, marker: &gc::Marker) {
        self.values.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::TextShadow::Shadow", mark)]
pub struct YTextShadowShadow {
    color: CachedValue<Option<Color>>,
    horizontal: CachedValue<Length>,
    vertical: CachedValue<Length>,
    blur: CachedValue<Option<NonNegative<Length>>>,
}

impl YTextShadowShadow {
    pub fn new(value: SingleSpecifiedValue) -> Self {
        Self {
            color: CachedValue::new(value.color, |color, ruby| match color {
                Some(color) => make_color(color, ruby),
                None => ruby.qnil().into_value_with(ruby),
            }),

            horizontal: CachedValue::new(value.horizontal, |length, ruby| {
                length_to_value(length, ruby)
            }),

            vertical: CachedValue::new(value.vertical, |length, ruby| {
                length_to_value(length, ruby)
            }),

            blur: CachedValue::new(value.blur, |blur, ruby| match blur {
                Some(blur) => length_to_value(&blur.0, ruby),
                None => ruby.qnil().into_value_with(ruby),
            }),
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
}

impl DataTypeFunctions for YTextShadowShadow {
    fn mark(&self, marker: &gc::Marker) {
        self.color.mark(marker);
        self.horizontal.mark(marker);
        self.vertical.mark(marker);
        self.blur.mark(marker);
    }
}
