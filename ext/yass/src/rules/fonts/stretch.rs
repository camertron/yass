use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc, typed_data, value::Id};
use style::{font_face::FontStretchRange, values::specified::{FontStretch, Percentage, font::FontStretchKeyword}};

use crate::{cached_value::CachedValue, rules::fonts::YSystemFont};

pub fn font_stretch_to_value(stretch: &FontStretch, ruby: &Ruby) -> Value {
    match stretch {
        FontStretch::Stretch(non_negative) => {
            YFontStretch::new(&non_negative.0).into_value_with(ruby)
        },

        FontStretch::Keyword(font_stretch_keyword) => {
            YFontStretchKeyword::new(*font_stretch_keyword).into_value_with(ruby)
        },

        FontStretch::System(_) => {
            YSystemFont::new().into_value_with(ruby)
        },
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::FontStretch::Range", mark)]
pub struct YFontStretchRange {
    begin: CachedValue<FontStretch>,
    end: CachedValue<FontStretch>,
}

impl YFontStretchRange {
    pub fn new(range: FontStretchRange) -> Self {
        Self {
            begin: CachedValue::new(range.0, |stretch, ruby| {
                font_stretch_to_value(stretch, ruby)
            }),

            end: CachedValue::new(range.1, |stretch, ruby| {
                font_stretch_to_value(stretch, ruby)
            }),
        }
    }

    pub fn begin(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.begin.get(ruby)
    }

    pub fn end(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.end.get(ruby)
    }
}

impl DataTypeFunctions for YFontStretchRange {
    fn mark(&self, marker: &gc::Marker) {
        self.begin.mark(marker);
        self.end.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::FontStretch::Stretch")]
pub struct YFontStretch {
    value: f32
}

impl YFontStretch {
    pub fn new(percentage: &Percentage) -> Self {
        Self { value: percentage.get() }
    }

    pub fn value(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> f32 {
        rb_self.value
    }
}

#[magnus::wrap(class = "Yass::FontStretch::Keyword")]
pub struct YFontStretchKeyword {
    keyword: FontStretchKeyword
}

impl YFontStretchKeyword {
    pub fn new(keyword: FontStretchKeyword) -> Self {
        Self { keyword }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.keyword {
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
}
