use magnus::{IntoValue, Ruby, Value, typed_data};
use style::values::specified::BorderSideOffset;
use style_traits::ToCss;

#[magnus::wrap(class = "Yass::Declarations::OutlineOffset")]
pub struct YOutlineOffset {
    border_side_offset: BorderSideOffset
}

impl YOutlineOffset {
    pub fn new(border_side_offset: BorderSideOffset) -> Self {
        Self { border_side_offset }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        // BorderSideOffset wraps Length with a private field, so we have to use
        // to_css_string instead
        ruby
            .str_new(&rb_self.border_side_offset.to_css_string())
            .into_value_with(ruby)
    }
}
