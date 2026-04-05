use magnus::{IntoValue, Ruby, Value, typed_data};
use style::values::specified::NonNegativeNumber;

use crate::declarations::number::YNumber;

#[magnus::wrap(class = "Yass::Declarations::FlexGrow")]
pub struct YFlexGrow {
    non_negative_number: NonNegativeNumber
}

impl YFlexGrow {
    pub fn new(non_negative_number: NonNegativeNumber) -> Self {
        Self { non_negative_number }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        YNumber::new(rb_self.non_negative_number.get()).into_value_with(ruby)
    }
}
