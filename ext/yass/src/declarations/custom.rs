use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc, typed_data, value::Id};
use style::{
    custom_properties::SpecifiedValue as CustomSpecifiedValue,
    properties::{CSSWideKeyword, CustomDeclaration, CustomDeclarationValue},
    properties_and_values::value::SpecifiedValue as RegisteredCustomSpecifiedValue,
};
use style_traits::ToCss;

use crate::{cached_value::CachedValue, declarations::csswide_keyword::css_wide_keyword_to_id};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Custom", mark)]
pub struct YCustom {
    name: CachedValue<String>,
    value: CachedValue<CustomDeclarationValue>,
}

impl YCustom {
    pub fn new(custom_declaration: CustomDeclaration) -> Self {
        Self {
            name: CachedValue::new(custom_declaration.name.to_string(), |name, ruby| {
                ruby.str_new(name).into_value_with(ruby)
            }),

            value: CachedValue::new(custom_declaration.value, |value, ruby| {
                make_custom_value(value, ruby)
            }),
        }
    }

    pub fn name(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.name.get(ruby)
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.value.get(ruby)
    }
}

impl DataTypeFunctions for YCustom {
    fn mark(&self, marker: &gc::Marker) {
        self.name.mark(marker);
        self.value.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::Declarations::Custom::Unparsed")]
pub struct YCustomValueUnparsed {
    value: CustomSpecifiedValue,
}

impl YCustomValueUnparsed {
    pub fn new(value: CustomSpecifiedValue) -> Self {
        Self { value }
    }

    pub fn value(ruby: &Ruby, rb_self: &Self) -> Value {
        ruby.str_new(&rb_self.value.css).into_value_with(ruby)
    }
}

#[magnus::wrap(class = "Yass::Declarations::Custom::Parsed")]
pub struct YCustomValueParsed {
    value: RegisteredCustomSpecifiedValue,
}

impl YCustomValueParsed {
    pub fn new(value: RegisteredCustomSpecifiedValue) -> Self {
        Self { value }
    }

    pub fn value(ruby: &Ruby, rb_self: &Self) -> Value {
        ruby
            .str_new(&rb_self.value.to_css_string())
            .into_value_with(ruby)
    }
}

#[magnus::wrap(class = "Yass::Declarations::Custom::CSSWideKeyword")]
pub struct YCustomValueCSSWideKeyword {
    value: CSSWideKeyword,
}

impl YCustomValueCSSWideKeyword {
    pub fn new(value: CSSWideKeyword) -> Self {
        Self { value }
    }

    pub fn value(ruby: &Ruby, rb_self: &Self) -> Id {
        css_wide_keyword_to_id(ruby, rb_self.value)
    }
}

pub fn make_custom_value(value: &CustomDeclarationValue, ruby: &Ruby) -> Value {
    match value {
        CustomDeclarationValue::Unparsed(value) => {
            YCustomValueUnparsed::new((**value).clone()).into_value_with(ruby)
        },

        CustomDeclarationValue::Parsed(value) => {
            YCustomValueParsed::new((**value).clone()).into_value_with(ruby)
        },

        CustomDeclarationValue::CSSWideKeyword(value) => {
            YCustomValueCSSWideKeyword::new(*value).into_value_with(ruby)
        },
    }
}
