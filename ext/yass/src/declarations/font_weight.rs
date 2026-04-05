use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc, typed_data};
use style::values::specified::{FontWeight, font::AbsoluteFontWeight};

use crate::{cached_value::CachedValue, declarations::number::YNumber};

fn make_font_weight(font_weight: &FontWeight, ruby: &Ruby) -> Value {
    match font_weight {
        FontWeight::Absolute(absolute) => YFontWeightAbsolute::new(*absolute).into_value_with(ruby),
        FontWeight::Bolder => YFontWeightBolder::new().into_value_with(ruby),
        FontWeight::Lighter => YFontWeightLighter::new().into_value_with(ruby),
        FontWeight::System(_) => YFontWeightSystem::new().into_value_with(ruby),
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::FontWeight", mark)]
pub struct YFontWeight {
    font_weight: CachedValue<FontWeight>
}

impl YFontWeight {
    pub fn new(font_weight: FontWeight) -> Self {
        Self {
            font_weight: CachedValue::new(font_weight, make_font_weight),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.font_weight.get(ruby)
    }
}

impl DataTypeFunctions for YFontWeight {
    fn mark(&self, marker: &gc::Marker) {
        self.font_weight.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::Declarations::FontWeight::Absolute")]
pub struct YFontWeightAbsolute {
    absolute_font_weight: AbsoluteFontWeight,
}

impl YFontWeightAbsolute {
    pub fn new(absolute_font_weight: AbsoluteFontWeight) -> Self {
        Self {
            absolute_font_weight,
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        match rb_self.absolute_font_weight {
            AbsoluteFontWeight::Weight(number) => YNumber::new(number.get()).into_value_with(ruby),
            AbsoluteFontWeight::Normal => ruby.intern("normal").into_value_with(ruby),
            AbsoluteFontWeight::Bold => ruby.intern("bold").into_value_with(ruby),
        }
    }
}

#[magnus::wrap(class = "Yass::Declarations::FontWeight::Bolder")]
pub struct YFontWeightBolder {}

impl YFontWeightBolder {
    pub fn new() -> Self {
        Self {}
    }
}

#[magnus::wrap(class = "Yass::Declarations::FontWeight::Lighter")]
pub struct YFontWeightLighter {}

impl YFontWeightLighter {
    pub fn new() -> Self {
        Self {}
    }
}

#[magnus::wrap(class = "Yass::Declarations::FontWeight::System")]
pub struct YFontWeightSystem {}

impl YFontWeightSystem {
    pub fn new() -> Self {
        Self {}
    }
}
