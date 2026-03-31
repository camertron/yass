use magnus::{Ruby, typed_data, value::Id};
use style::properties::longhands::border_collapse::SpecifiedValue;

#[magnus::wrap(class = "Yass::Declarations::BorderCollapse")]
pub struct YBorderCollapse {
    specified_value: SpecifiedValue
}

impl YBorderCollapse {
    pub fn new(specified_value: SpecifiedValue) -> Self {
        Self { specified_value }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.specified_value {
            SpecifiedValue::Separate => ruby.intern("separate"),
            SpecifiedValue::Collapse => ruby.intern("collapse"),
        }
    }
}
