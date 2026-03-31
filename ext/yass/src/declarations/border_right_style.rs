use magnus::{Ruby, typed_data, value::Id};
use style::values::computed::BorderStyle;

use crate::declarations::border_style::border_style_to_id;

#[magnus::wrap(class = "Yass::Declarations::BorderRightStyle")]
pub struct YBorderRightStyle {
    border_style: BorderStyle
}

impl YBorderRightStyle {
    pub fn new(border_style: BorderStyle) -> Self {
        Self { border_style }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        border_style_to_id(rb_self.border_style, ruby)
    }
}
