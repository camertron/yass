use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc, typed_data};
use style::values::specified::Opacity;
use style_traits::ToCss;

use crate::{cached_value::CachedValue, declarations::number::YNumber};

fn opacity_to_value(opacity: &Opacity, ruby: &Ruby) -> Value {
    let css = opacity.to_css_string();

    if let Ok(number) = css.parse::<f32>() {
        return YNumber::new(number).into_value_with(ruby);
    }

    ruby.str_new(&css).into_value_with(ruby)
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Opacity", mark)]
pub struct YOpacity {
    value: CachedValue<Opacity>,
}

impl YOpacity {
    pub fn new(opacity: Opacity) -> Self {
        Self {
            value: CachedValue::new(opacity, opacity_to_value),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.value.get(ruby)
    }
}

impl DataTypeFunctions for YOpacity {
    fn mark(&self, marker: &gc::Marker) {
        self.value.mark(marker);
    }
}
