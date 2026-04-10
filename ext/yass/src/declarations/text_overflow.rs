use magnus::{DataTypeFunctions, IntoValue, RString, Ruby, TypedData, Value, gc, typed_data};
use style::values::{computed::TextOverflow, specified::text::TextOverflowSide};

use crate::cached_value::CachedValue;

fn make_text_overflow_side(side: &TextOverflowSide, ruby: &Ruby) -> Value {
    match side {
        TextOverflowSide::Clip => YTextOverflowClip::new().into_value_with(ruby),
        TextOverflowSide::Ellipsis => YTextOverflowEllipsis::new().into_value_with(ruby),
        TextOverflowSide::String(value) => {
            YTextOverflowString::new(value.to_string()).into_value_with(ruby)
        }
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::TextOverflow", mark)]
pub struct YTextOverflow {
    first: CachedValue<TextOverflowSide>,
    second: CachedValue<TextOverflowSide>,
    sides_are_logical: bool,
}

impl YTextOverflow {
    pub fn new(specified_value: Box<TextOverflow>) -> Self {
        Self {
            first: CachedValue::new(specified_value.first.clone(), make_text_overflow_side),
            second: CachedValue::new(specified_value.second.clone(), make_text_overflow_side),
            sides_are_logical: specified_value.sides_are_logical,
        }
    }

    pub fn first(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.first.get(ruby)
    }

    pub fn second(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.second.get(ruby)
    }

    pub fn sides_are_logical(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.sides_are_logical
    }
}

impl DataTypeFunctions for YTextOverflow {
    fn mark(&self, marker: &gc::Marker) {
        self.first.mark(marker);
        self.second.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::Declarations::TextOverflow::Clip")]
pub struct YTextOverflowClip {}

impl YTextOverflowClip {
    pub fn new() -> Self {
        Self {}
    }
}

#[magnus::wrap(class = "Yass::Declarations::TextOverflow::Ellipsis")]
pub struct YTextOverflowEllipsis {}

impl YTextOverflowEllipsis {
    pub fn new() -> Self {
        Self {}
    }
}

#[magnus::wrap(class = "Yass::Declarations::TextOverflow::String")]
pub struct YTextOverflowString {
    value: String,
}

impl YTextOverflowString {
    pub fn new(value: String) -> Self {
        Self { value }
    }

    pub fn value(ruby: &Ruby, rb_self: &Self) -> RString {
        ruby.str_new(&rb_self.value)
    }
}
