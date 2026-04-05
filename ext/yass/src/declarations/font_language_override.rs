use magnus::{IntoValue, Ruby, Value, typed_data};
use style::values::computed::FontLanguageOverride;

#[magnus::wrap(class = "Yass::Declarations::FontLanguageOverride")]
pub struct YFontLanguageOverride {
    font_language_override: FontLanguageOverride
}

impl YFontLanguageOverride {
    pub fn new(font_language_override: FontLanguageOverride) -> Self {
        Self { font_language_override }
    }

    pub fn is_normal(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.font_language_override.0 == 0
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        if rb_self.font_language_override.0 == 0 {
            return ruby.intern("normal").into_value_with(ruby);
        }

        let bytes = rb_self.font_language_override.0.to_be_bytes();
        let value = std::str::from_utf8(&bytes)
            .unwrap_or_default()
            .trim_end();

        ruby.str_new(value).into_value_with(ruby)
    }
}
