use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc::Marker, typed_data};
use style::values::specified::LetterSpacing;
use style::values::specified::text::Spacing;

use crate::{cached_value::CachedValue, declarations::size::length_percentage_to_value};

fn letter_spacing_to_value(letter_spacing: &LetterSpacing, ruby: &Ruby) -> Value {
    match &letter_spacing.0 {
        Spacing::Normal => YLetterSpacingNormal::new().into_value_with(ruby),
        Spacing::Value(length_percentage) => {
            length_percentage_to_value(length_percentage, ruby)
        }
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::LetterSpacing", mark)]
pub struct YLetterSpacing {
    value: CachedValue<LetterSpacing>,
}

impl YLetterSpacing {
    pub fn new(letter_spacing: LetterSpacing) -> Self {
        Self {
            value: CachedValue::new(letter_spacing, letter_spacing_to_value),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.value.get(ruby)
    }
}

impl DataTypeFunctions for YLetterSpacing {
    fn mark(&self, marker: &Marker) {
        self.value.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::Declarations::LetterSpacing::Normal")]
pub struct YLetterSpacingNormal {}

impl YLetterSpacingNormal {
    pub fn new() -> Self {
        Self {}
    }
}
