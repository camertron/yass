use magnus::{Ruby, typed_data, value::Id};
use style::properties::longhands::box_sizing::SpecifiedValue;

#[magnus::wrap(class = "Yass::Declarations::BoxSizing")]
pub struct YBoxSizing {
    specified_value: SpecifiedValue
}

impl YBoxSizing {
    pub fn new(specified_value: SpecifiedValue) -> Self {
        Self { specified_value }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.specified_value {
            SpecifiedValue::ContentBox => ruby.intern("content_box"),
            SpecifiedValue::BorderBox => ruby.intern("border_box"),
        }
    }
}
