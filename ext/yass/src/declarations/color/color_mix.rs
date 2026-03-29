use magnus::{DataTypeFunctions, Error, IntoValue, RArray, Ruby, TypedData, Value, gc, typed_data};
use style::values::generics::color::GenericColorMixItem;
use style::values::specified::Color;
use style::values::specified::percentage::Percentage;
use style::values::{
    generics::color::ColorMixFlags,
    specified::color::ColorMix as SpecifiedColorMix,
};
use style::color::mix::ColorInterpolationMethod;

use crate::declarations::color::color_interpolation_method::YColorInterpolationMethod;
use crate::declarations::color::color_mix_item::YColorMixItem;
use crate::{cached_value::CachedValue, cached_value_list::CachedValueList};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Color::ColorMix", mark)]
pub struct YColorMix {
    flags: ColorMixFlags,
    interpolation: CachedValue<ColorInterpolationMethod>,
    items: CachedValueList<GenericColorMixItem<Color, Percentage>>,
}

impl YColorMix {
    pub fn new(color_mix: SpecifiedColorMix) -> Self {
        Self {
            flags: color_mix.flags,

            interpolation: CachedValue::new(color_mix.interpolation, |interpolation, ruby| {
                YColorInterpolationMethod::new(*interpolation).into_value_with(ruby)
            }),

            items: CachedValueList::new(color_mix.items.to_vec(), |item, _ctx, ruby| {
                YColorMixItem::new(item.clone()).into_value_with(ruby)
            }),
        }
    }

    pub fn interpolation(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.interpolation.get(ruby)
    }

    pub fn items(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        rb_self.items.to_a(ruby)
    }

    pub fn normalize_weights(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.flags.contains(ColorMixFlags::NORMALIZE_WEIGHTS)
    }

    pub fn result_in_modern_syntax(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.flags.contains(ColorMixFlags::RESULT_IN_MODERN_SYNTAX)
    }
}

impl DataTypeFunctions for YColorMix {
    fn mark(&self, marker: &gc::Marker) {
        self.interpolation.mark(marker);
        self.items.mark(marker);
    }
}
