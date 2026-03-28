use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc, typed_data};
use style::values::{
    generics::NonNegative,
    specified::{Color, ColorPropertyValue, Length, effects::SimpleShadow},
};

use crate::{cached_value::CachedValue, declarations::YColor};

use super::length_to_value;

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Filter::DropShadow", mark)]
pub struct YFilterDropShadow {
    color: CachedValue<Option<Color>>,
    horizontal: CachedValue<Length>,
    vertical: CachedValue<Length>,
    blur: CachedValue<Option<NonNegative<Length>>>,
}

impl YFilterDropShadow {
    pub fn new(shadow: SimpleShadow) -> Self {
        Self {
            color: CachedValue::new(shadow.color, |color, ruby| match color {
                Some(color) => YColor::new(ColorPropertyValue(color.clone())).into_value_with(ruby),
                None => ruby.qnil().into_value_with(ruby),
            }),

            horizontal: CachedValue::new(shadow.horizontal, |horizontal, ruby| {
                length_to_value(horizontal.clone(), ruby)
            }),

            vertical: CachedValue::new(shadow.vertical, |vertical, ruby| {
                length_to_value(vertical.clone(), ruby)
            }),

            blur: CachedValue::new(shadow.blur, |blur, ruby| match blur {
                Some(length) => length_to_value(length.0.clone(), ruby),
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

impl DataTypeFunctions for YFilterDropShadow {
    fn mark(&self, marker: &gc::Marker) {
        self.color.mark(marker);
        self.horizontal.mark(marker);
        self.vertical.mark(marker);
        self.blur.mark(marker);
    }
}
