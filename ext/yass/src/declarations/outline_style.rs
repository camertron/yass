use style::values::computed::OutlineStyle;
use magnus::{Ruby, typed_data, value::Id};

use crate::declarations::border_style::border_style_to_id;

#[magnus::wrap(class = "Yass::Declarations::OutlineStyle")]
pub struct YOutlineStyle {
    outline_style: OutlineStyle
}

impl YOutlineStyle {
    pub fn new(outline_style: OutlineStyle) -> Self {
        Self { outline_style }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.outline_style {
            OutlineStyle::Auto => ruby.intern("auto"),
            OutlineStyle::BorderStyle(border_style) => border_style_to_id(border_style, ruby),
        }
    }
}
