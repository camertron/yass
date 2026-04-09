use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc, typed_data};
use style::values::{generics::{NonNegative, Optional, length::{AnchorSizeKeyword, GenericAnchorSizeFunction, MaxSize}}, specified::LengthPercentage};

use crate::{cached_value::CachedValue, declarations::size::{anchor_size_keyword::YAnchorSizeKeyword, make_max_size}};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Size::AnchorMaxSizeFunction", mark)]
pub struct YAnchorMaxSizeFunction {
    target_element: CachedValue<String>,
    size: CachedValue<AnchorSizeKeyword>,
    fallback: CachedValue<Optional<MaxSize<NonNegative<LengthPercentage>>>>
}

impl YAnchorMaxSizeFunction {
    pub fn new(anchor_size: GenericAnchorSizeFunction<MaxSize<NonNegative<LengthPercentage>>>) -> Self {
        Self {
            target_element: CachedValue::new(anchor_size.target_element.value.0.to_string(), |el, ruby| {
                ruby.str_new(el).into_value_with(ruby)
            }),

            size: CachedValue::new(anchor_size.size, |size, ruby| {
                YAnchorSizeKeyword::new(*size).into_value_with(ruby)
            }),

            fallback: CachedValue::new(anchor_size.fallback, |fallback, ruby| {
                match fallback {
                    Optional::Some(fb) => make_max_size(fb, ruby).into_value_with(ruby),
                    Optional::None => ruby.qnil().into_value_with(ruby)
                }
            })
        }
    }

    pub fn target_element(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.target_element.get(ruby)
    }

    pub fn size(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.size.get(ruby)
    }

    pub fn fallback(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.fallback.get(ruby)
    }
}

impl DataTypeFunctions for YAnchorMaxSizeFunction {
    fn mark(&self, marker: &gc::Marker) {
        self.target_element.mark(marker);
        self.size.mark(marker);
        self.fallback.mark(marker);
    }
}
