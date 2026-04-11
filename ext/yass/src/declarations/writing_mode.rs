use magnus::{Ruby, typed_data, value::Id};
use style::values::computed::WritingModeProperty;

#[magnus::wrap(class = "Yass::Declarations::WritingMode")]
pub struct YWritingMode {
    writing_mode_property: WritingModeProperty
}

impl YWritingMode {
    pub fn new(writing_mode_property: WritingModeProperty) -> Self {
        Self { writing_mode_property }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.writing_mode_property {
            WritingModeProperty::HorizontalTb => ruby.intern("horizontal_tb"),
            WritingModeProperty::VerticalRl => ruby.intern("vertical_rl"),
            WritingModeProperty::VerticalLr => ruby.intern("vertical_lr"),
        }
    }
}
