use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc, typed_data};
use style::values::specified::AngleOrPercentage;

use crate::{cached_value::CachedValue, declarations::{angle::YAngle, percentage::YPercentage}};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Image::AngleOrPercentage", mark)]
pub struct YAngleOrPercentage {
    value: CachedValue<AngleOrPercentage>,
}

impl YAngleOrPercentage {
    pub fn new(value: AngleOrPercentage) -> Self {
        Self {
            value: CachedValue::new(value, |val, ruby| {
                match val {
                    AngleOrPercentage::Angle(angle) => YAngle::new(*angle).into_value_with(ruby),
                    AngleOrPercentage::Percentage(percentage) => {
                        YPercentage::new(percentage.get()).into_value_with(ruby)
                    },
                }
            })
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.value.get(ruby)
    }
}

impl DataTypeFunctions for YAngleOrPercentage {
    fn mark(&self, marker: &gc::Marker) {
        self.value.mark(marker);
    }
}
