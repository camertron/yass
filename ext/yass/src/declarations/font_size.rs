use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc, typed_data, value::Id};
use style::values::specified::{FontSize, LengthPercentage, font::{FontSizeKeyword, KeywordInfo}};

use crate::{cached_value::CachedValue, declarations::size::YLengthPercentage};

fn make_font_size(font_size: &FontSize, ruby: &Ruby) -> Value {
    match font_size {
        FontSize::Length(length_percentage) => {
            YFontSizeLength::new(length_percentage.clone()).into_value_with(ruby)
        },
        FontSize::Keyword(keyword_info) => YFontSizeKeyword::new(*keyword_info).into_value_with(ruby),
        FontSize::Smaller => YFontSizeSmaller::new().into_value_with(ruby),
        FontSize::Larger => YFontSizeLarger::new().into_value_with(ruby),
        FontSize::System(_) => YFontSizeSystem::new().into_value_with(ruby),
    }
}

fn font_size_keyword_to_id(keyword: FontSizeKeyword, ruby: &Ruby) -> Id {
    match keyword {
        FontSizeKeyword::XXSmall => ruby.intern("xx_small"),
        FontSizeKeyword::XSmall => ruby.intern("x_small"),
        FontSizeKeyword::Small => ruby.intern("small"),
        FontSizeKeyword::Medium => ruby.intern("medium"),
        FontSizeKeyword::Large => ruby.intern("large"),
        FontSizeKeyword::XLarge => ruby.intern("x_large"),
        FontSizeKeyword::XXLarge => ruby.intern("xx_large"),
        FontSizeKeyword::XXXLarge => ruby.intern("xxx_large"),
        FontSizeKeyword::None => ruby.intern("none"),
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::FontSize", mark)]
pub struct YFontSize {
    font_size: CachedValue<FontSize>
}

impl YFontSize {
    pub fn new(font_size: FontSize) -> Self {
        Self {
            font_size: CachedValue::new(font_size, make_font_size),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.font_size.get(ruby)
    }
}

impl DataTypeFunctions for YFontSize {
    fn mark(&self, marker: &gc::Marker) {
        self.font_size.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::FontSize::Length", mark)]
pub struct YFontSizeLength {
    length_percentage: CachedValue<LengthPercentage>,
}

impl YFontSizeLength {
    pub fn new(length_percentage: LengthPercentage) -> Self {
        Self {
            length_percentage: CachedValue::new(length_percentage, |value, ruby| {
                YLengthPercentage::new(value.clone()).into_value_with(ruby)
            }),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.length_percentage.get(ruby)
    }
}

impl DataTypeFunctions for YFontSizeLength {
    fn mark(&self, marker: &gc::Marker) {
        self.length_percentage.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::Declarations::FontSize::Keyword")]
pub struct YFontSizeKeyword {
    keyword_info: KeywordInfo
}

impl YFontSizeKeyword {
    pub fn new(keyword_info: KeywordInfo) -> Self {
        Self { keyword_info }
    }

    pub fn keyword(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        font_size_keyword_to_id(rb_self.keyword_info.kw, ruby)
    }

    pub fn factor(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> f32 {
        rb_self.keyword_info.factor
    }
}

#[magnus::wrap(class = "Yass::Declarations::FontSize::Smaller")]
pub struct YFontSizeSmaller {}

impl YFontSizeSmaller {
    pub fn new() -> Self {
        Self {}
    }
}

#[magnus::wrap(class = "Yass::Declarations::FontSize::Larger")]
pub struct YFontSizeLarger {}

impl YFontSizeLarger {
    pub fn new() -> Self {
        Self {}
    }
}

#[magnus::wrap(class = "Yass::Declarations::FontSize::System")]
pub struct YFontSizeSystem {}

impl YFontSizeSystem {
    pub fn new() -> Self {
        Self {}
    }
}
