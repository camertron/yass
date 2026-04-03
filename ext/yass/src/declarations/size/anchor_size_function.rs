use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc, typed_data, value::Id};
use style::values::{generics::{NonNegative, Optional, length::{AnchorSizeKeyword, GenericAnchorSizeFunction, Size}}, specified::LengthPercentage};

use crate::{cached_value::CachedValue, declarations::size::make_size};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Size::AnchorSizeFunction", mark)]
pub struct YAnchorSizeFunction {
    target_element: CachedValue<String>,
    size: CachedValue<AnchorSizeKeyword>,
    fallback: CachedValue<Optional<Size<NonNegative<LengthPercentage>>>>
}

impl YAnchorSizeFunction {
    pub fn new(anchor_size: GenericAnchorSizeFunction<Size<NonNegative<LengthPercentage>>>) -> Self {
        Self {
            target_element: CachedValue::new(anchor_size.target_element.value.0.to_string(), |el, ruby| {
                ruby.str_new(el).into_value_with(ruby)
            }),

            size: CachedValue::new(anchor_size.size, |size, ruby| {
                YAnchorSizeKeyword::new(*size).into_value_with(ruby)
            }),

            fallback: CachedValue::new(anchor_size.fallback, |fallback, ruby| {
                match fallback {
                    Optional::Some(fb) => {
                        make_size(fb.clone(), ruby).into_value_with(ruby)
                    },

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

impl DataTypeFunctions for YAnchorSizeFunction {
    fn mark(&self, marker: &gc::Marker) {
        self.target_element.mark(marker);
        self.size.mark(marker);
        self.fallback.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::Declarations::Calc::AnchorSizeKeyword")]
pub struct YAnchorSizeKeyword {
    anchor_size_keyword: AnchorSizeKeyword
}

impl YAnchorSizeKeyword {
    pub fn new(anchor_size_keyword: AnchorSizeKeyword) -> Self {
        Self { anchor_size_keyword }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.anchor_size_keyword {
            AnchorSizeKeyword::None => unreachable!(),
            AnchorSizeKeyword::Width => ruby.intern("width"),
            AnchorSizeKeyword::Height => ruby.intern("height"),
            AnchorSizeKeyword::Block => ruby.intern("block"),
            AnchorSizeKeyword::Inline => ruby.intern("inline"),
            AnchorSizeKeyword::SelfBlock => ruby.intern("self_block"),
            AnchorSizeKeyword::SelfInline => ruby.intern("self_inline"),
        }
    }
}
