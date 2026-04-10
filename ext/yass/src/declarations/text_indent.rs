use magnus::{DataTypeFunctions, Ruby, TypedData, Value, gc, typed_data};
use style::values::{generics::text::GenericTextIndent, specified::LengthPercentage};

use crate::{cached_value::CachedValue, declarations::size::length_percentage_to_value};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::TextIndent", mark)]
pub struct YTextIndent {
    length: CachedValue<LengthPercentage>,
    hanging: bool,
    each_line: bool,
}

impl YTextIndent {
    pub fn new(text_indent: GenericTextIndent<LengthPercentage>) -> Self {
        Self {
            length: CachedValue::new(text_indent.length, length_percentage_to_value),
            hanging: text_indent.hanging,
            each_line: text_indent.each_line,
        }
    }

    pub fn length(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.length.get(ruby)
    }

    pub fn hanging(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.hanging
    }

    pub fn each_line(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.each_line
    }
}

impl DataTypeFunctions for YTextIndent {
    fn mark(&self, marker: &gc::Marker) {
        self.length.mark(marker);
    }
}
