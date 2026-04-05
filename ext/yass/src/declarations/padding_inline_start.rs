use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc, typed_data};
use style::values::specified::NonNegativeLengthPercentage;

use crate::{cached_value::CachedValue, declarations::size::YLengthPercentage};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::PaddingInlineStart", mark)]
pub struct YPaddingInlineStart {
  value: CachedValue<NonNegativeLengthPercentage>,
}

impl YPaddingInlineStart {
  pub fn new(value: NonNegativeLengthPercentage) -> Self {
    Self {
      value: CachedValue::new(value, |non_negative, ruby| {
        YLengthPercentage::new(non_negative.0.clone()).into_value_with(ruby)
      }),
    }
  }

  pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
    rb_self.value.get(ruby)
  }
}

impl DataTypeFunctions for YPaddingInlineStart {
  fn mark(&self, marker: &gc::Marker) {
    self.value.mark(marker);
  }
}
