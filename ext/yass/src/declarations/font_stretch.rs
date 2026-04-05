use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc, typed_data, value::Id};
use style::values::specified::{FontStretch, NonNegativePercentage, font::FontStretchKeyword};

use crate::{cached_value::CachedValue, declarations::percentage::YPercentage};

fn font_stretch_keyword_to_id(keyword: FontStretchKeyword, ruby: &Ruby) -> Id {
    match keyword {
        FontStretchKeyword::Normal => ruby.intern("normal"),
        FontStretchKeyword::Condensed => ruby.intern("condensed"),
        FontStretchKeyword::UltraCondensed => ruby.intern("ultra_condensed"),
        FontStretchKeyword::ExtraCondensed => ruby.intern("extra_condensed"),
        FontStretchKeyword::SemiCondensed => ruby.intern("semi_condensed"),
        FontStretchKeyword::SemiExpanded => ruby.intern("semi_expanded"),
        FontStretchKeyword::Expanded => ruby.intern("expanded"),
        FontStretchKeyword::ExtraExpanded => ruby.intern("extra_expanded"),
        FontStretchKeyword::UltraExpanded => ruby.intern("ultra_expanded"),
    }
}

fn make_font_stretch(font_stretch: &FontStretch, ruby: &Ruby) -> Value {
    match font_stretch {
        FontStretch::Stretch(percentage) => {
            YFontStretchValue::new(*percentage).into_value_with(ruby)
        },
        FontStretch::Keyword(keyword) => YFontStretchKeyword::new(*keyword).into_value_with(ruby),
        FontStretch::System(_) => YFontStretchSystem::new().into_value_with(ruby),
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::FontStretch", mark)]
pub struct YFontStretch {
    font_stretch: CachedValue<FontStretch>
}

impl YFontStretch {
    pub fn new(font_stretch: FontStretch) -> Self {
        Self {
            font_stretch: CachedValue::new(font_stretch, make_font_stretch),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.font_stretch.get(ruby)
    }
}

impl DataTypeFunctions for YFontStretch {
    fn mark(&self, marker: &gc::Marker) {
        self.font_stretch.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::FontStretch::Stretch", mark)]
pub struct YFontStretchValue {
    percentage: CachedValue<NonNegativePercentage>,
}

impl YFontStretchValue {
    pub fn new(percentage: NonNegativePercentage) -> Self {
        Self {
            percentage: CachedValue::new(percentage, |value, ruby| {
                YPercentage::new(value.0.get()).into_value_with(ruby)
            }),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.percentage.get(ruby)
    }
}

impl DataTypeFunctions for YFontStretchValue {
    fn mark(&self, marker: &gc::Marker) {
        self.percentage.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::Declarations::FontStretch::Keyword")]
pub struct YFontStretchKeyword {
    keyword: FontStretchKeyword,
}

impl YFontStretchKeyword {
    pub fn new(keyword: FontStretchKeyword) -> Self {
        Self { keyword }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        font_stretch_keyword_to_id(rb_self.keyword, ruby)
    }
}

#[magnus::wrap(class = "Yass::Declarations::FontStretch::System")]
pub struct YFontStretchSystem {}

impl YFontStretchSystem {
    pub fn new() -> Self {
        Self {}
    }
}
