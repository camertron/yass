use magnus::{IntoValue, Ruby, Value, typed_data};
use style::values::{
    generics::{
        NonNegative,
        position::{AspectRatio, PreferredRatio},
    },
    specified::Number,
};

use crate::declarations::number::YNumber;

#[magnus::wrap(class = "Yass::Declarations::AspectRatio")]
pub struct YAspectRatio {
    aspect_ratio: AspectRatio<NonNegative<Number>>
}

impl YAspectRatio {
    pub fn new(aspect_ratio: AspectRatio<NonNegative<Number>>) -> Self {
        Self { aspect_ratio }
    }

    pub fn is_auto(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.aspect_ratio.auto
    }

    pub fn has_ratio(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        matches!(rb_self.aspect_ratio.ratio, PreferredRatio::Ratio(_))
    }

    pub fn numerator(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Option<Value> {
        match rb_self.aspect_ratio.ratio {
            PreferredRatio::Ratio(ratio) => Some(YNumber::new(ratio.0.0.get()).into_value_with(ruby)),
            PreferredRatio::None => None,
        }
    }

    pub fn denominator(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Option<Value> {
        match rb_self.aspect_ratio.ratio {
            PreferredRatio::Ratio(ratio) => Some(YNumber::new(ratio.1.0.get()).into_value_with(ruby)),
            PreferredRatio::None => None,
        }
    }
}
