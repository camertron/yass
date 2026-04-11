use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc::Marker, typed_data};
use style::values::specified::WordSpacing;
use style::values::specified::text::Spacing;

use crate::{cached_value::CachedValue, declarations::size::length_percentage_to_value};

fn word_spacing_to_value(word_spacing: &WordSpacing, ruby: &Ruby) -> Value {
    match &word_spacing.0 {
        Spacing::Normal => YWordSpacingNormal::new().into_value_with(ruby),
        Spacing::Value(length_percentage) => {
            length_percentage_to_value(length_percentage, ruby)
        }
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::WordSpacing", mark)]
pub struct YWordSpacing {
    value: CachedValue<WordSpacing>,
}

impl YWordSpacing {
    pub fn new(word_spacing: WordSpacing) -> Self {
        Self {
            value: CachedValue::new(word_spacing, word_spacing_to_value),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.value.get(ruby)
    }
}

impl DataTypeFunctions for YWordSpacing {
    fn mark(&self, marker: &Marker) {
        self.value.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::Declarations::WordSpacing::Normal")]
pub struct YWordSpacingNormal {}

impl YWordSpacingNormal {
    pub fn new() -> Self {
        Self {}
    }
}
