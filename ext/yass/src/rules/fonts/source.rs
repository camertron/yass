use magnus::{DataTypeFunctions, IntoValue, RString, Ruby, TypedData, Value, gc, typed_data, value::Id};
use style::{font_face::{FontFaceSourceFormat, FontFaceSourceFormatKeyword, Source, UrlSource}, values::{computed::font::FamilyName}};

use crate::{cached_value::CachedValue, optional_cached_value::OptionalCachedValue, rules::fonts::family::YFontFamilyName};

pub fn font_source_to_value(source: &Source, ruby: &Ruby) -> Value {
    match source {
        Source::Url(url_source) => {
            YFontSourceUrl::new(url_source.clone()).into_value_with(ruby)
        },

        Source::Local(family_name) => {
            YFontSourceLocal::new(family_name.clone()).into_value_with(ruby)
        },
    }
}

pub fn font_source_format_to_value(source_format: &FontFaceSourceFormat, ruby: &Ruby) -> Value {
    match source_format {
        FontFaceSourceFormat::Keyword(keyword) => {
            YFontFaceSourceFormatKeyword::new(*keyword).into_value_with(ruby)
        },

        FontFaceSourceFormat::String(str) => {
            YFontFaceSourceFormatString::new(str.clone()).into_value_with(ruby)
        },
    }
}

#[magnus::wrap(class = "Yass::Font::SourceFormat::Keyword")]
pub struct YFontFaceSourceFormatKeyword {
    keyword: FontFaceSourceFormatKeyword
}

impl YFontFaceSourceFormatKeyword {
    pub fn new(keyword: FontFaceSourceFormatKeyword) -> Self {
        Self { keyword }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.keyword {
            FontFaceSourceFormatKeyword::None => ruby.intern("none"),
            FontFaceSourceFormatKeyword::Collection => ruby.intern("collection"),
            FontFaceSourceFormatKeyword::EmbeddedOpentype => ruby.intern("embedded_opentype"),
            FontFaceSourceFormatKeyword::Opentype => ruby.intern("opentype"),
            FontFaceSourceFormatKeyword::Svg => ruby.intern("svg"),
            FontFaceSourceFormatKeyword::Truetype => ruby.intern("truetype"),
            FontFaceSourceFormatKeyword::Woff => ruby.intern("woff"),
            FontFaceSourceFormatKeyword::Woff2 => ruby.intern("woff2"),
            FontFaceSourceFormatKeyword::Unknown => ruby.intern("unknown"),
        }
    }
}

#[magnus::wrap(class = "Yass::Font::SourceFormat::String")]
pub struct YFontFaceSourceFormatString {
    str: String
}

impl YFontFaceSourceFormatString {
    pub fn new(str: String) -> Self {
        Self { str }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> RString {
        ruby.str_new(&rb_self.str)
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Font::Source::Url", mark)]
pub struct YFontSourceUrl {
    specified_url: String,
    format_hint: OptionalCachedValue<FontFaceSourceFormat>,
}

impl YFontSourceUrl {
    pub fn new(url_source: UrlSource) -> Self {
        Self {
            specified_url: url_source.url.as_str().to_string(),
            format_hint: OptionalCachedValue::new(url_source.format_hint, |format_hint, ruby| {
                font_source_format_to_value(format_hint, ruby).into_value_with(ruby)
            })
        }
    }

    pub fn specified_url(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> RString {
        ruby.str_new(&rb_self.specified_url)
    }

    pub fn format_hint(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.format_hint.get(ruby)
    }
}

impl DataTypeFunctions for YFontSourceUrl {
    fn mark(&self, marker: &gc::Marker) {
        self.format_hint.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Font::Source::Local", mark)]
pub struct YFontSourceLocal {
    family_name: CachedValue<FamilyName>
}

impl YFontSourceLocal {
    pub fn new(family_name: FamilyName) -> Self {
        Self {
            family_name: CachedValue::new(family_name, |family_name, ruby| {
                YFontFamilyName::new(family_name.clone()).into_value_with(ruby)
            })
        }
    }

    pub fn family_name(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.family_name.get(ruby)
    }
}

impl DataTypeFunctions for YFontSourceLocal {
    fn mark(&self, marker: &gc::Marker) {
        self.family_name.mark(marker);
    }
}
