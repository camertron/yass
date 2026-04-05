use magnus::{DataTypeFunctions, Error, IntoValue, RArray, Ruby, TypedData, Value, gc, typed_data};
use style::values::{generics::font::VariationValue, specified::{FontVariationSettings, Number}};

use crate::{cached_value_list::CachedValueList, declarations::number::YNumber};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::FontVariationSettings", mark)]
pub struct YFontVariationSettings {
    font_variation_settings: CachedValueList<VariationValue<Number>>,
}

impl YFontVariationSettings {
    pub fn new(font_variation_settings: FontVariationSettings) -> Self {
        Self {
            font_variation_settings: CachedValueList::new(font_variation_settings.0.into_vec(), |setting, _ctx, ruby| {
                YFontVariationSetting::new(setting.clone()).into_value_with(ruby)
            }),
        }
    }

    pub fn values(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        rb_self.font_variation_settings.to_a(ruby)
    }

    pub fn is_normal(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.font_variation_settings.is_empty()
    }
}

impl DataTypeFunctions for YFontVariationSettings {
    fn mark(&self, marker: &gc::Marker) {
        self.font_variation_settings.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::Declarations::FontVariationSettings::Setting")]
pub struct YFontVariationSetting {
    variation_value: VariationValue<Number>,
}

impl YFontVariationSetting {
    pub fn new(variation_value: VariationValue<Number>) -> Self {
        Self { variation_value }
    }

    pub fn tag(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> String {
        let bytes = rb_self.variation_value.tag.0.to_be_bytes();
        String::from_utf8_lossy(&bytes).to_string()
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        YNumber::new(rb_self.variation_value.value.get()).into_value_with(ruby)
    }
}
