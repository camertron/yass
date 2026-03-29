use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc, typed_data};
use style::values::{
    generics::color::GenericColorMixItem,
    specified::{Color as SpecifiedColor, percentage::{Percentage, ToPercentage}},
};

use crate::{cached_value::CachedValue, declarations::{color::color::make_color, percentage::YPercentage}};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Color::ColorMixItem", mark)]
pub struct YColorMixItem {
    color: CachedValue<SpecifiedColor>,
    percentage: CachedValue<Percentage>,
}

impl YColorMixItem {
    pub fn new(item: GenericColorMixItem<SpecifiedColor, Percentage>) -> Self {
        Self {
            color: CachedValue::new(item.color, |color, ruby| {
                make_color(&color, ruby)
            }),

            percentage: CachedValue::new(item.percentage, |percentage, ruby| {
                YPercentage::new(percentage.to_percentage()).into_value_with(ruby)
            }),
        }
    }

    pub fn color(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.color.get(ruby)
    }

    pub fn percentage(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.percentage.get(ruby)
    }
}

impl DataTypeFunctions for YColorMixItem {
    fn mark(&self, marker: &gc::Marker) {
        self.color.mark(marker);
        self.percentage.mark(marker);
    }
}
