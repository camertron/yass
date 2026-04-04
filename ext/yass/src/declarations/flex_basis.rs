use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc, typed_data};
use style::values::{generics::{NonNegative, flex::FlexBasis, length::Size}, specified::LengthPercentage};

use crate::{cached_value::CachedValue, declarations::size::make_size};

fn make_flex_basis_value(
    value: &FlexBasis<Size<NonNegative<LengthPercentage>>>,
    ruby: &Ruby,
) -> Value {
    match value {
        FlexBasis::Content => YFlexBasisContent::new().into_value_with(ruby),
        FlexBasis::Size(size) => YFlexBasisSize::new(size.clone()).into_value_with(ruby),
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::FlexBasis", mark)]
pub struct YFlexBasis {
    value: CachedValue<FlexBasis<Size<NonNegative<LengthPercentage>>>>,
}

impl YFlexBasis {
    pub fn new(specified_value: Box<FlexBasis<Size<NonNegative<LengthPercentage>>>>) -> Self {
        Self {
            value: CachedValue::new((*specified_value).clone(), make_flex_basis_value),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.value.get(ruby)
    }
}

impl DataTypeFunctions for YFlexBasis {
    fn mark(&self, marker: &gc::Marker) {
        self.value.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::Declarations::FlexBasis::Content")]
pub struct YFlexBasisContent {}

impl YFlexBasisContent {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::FlexBasis::Size", mark)]
pub struct YFlexBasisSize {
    size: CachedValue<Size<NonNegative<LengthPercentage>>>,
}

impl YFlexBasisSize {
    pub fn new(size: Size<NonNegative<LengthPercentage>>) -> Self {
        Self {
            size: CachedValue::new(size, |size, ruby| make_size(size, ruby)),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.size.get(ruby)
    }
}

impl DataTypeFunctions for YFlexBasisSize {
    fn mark(&self, marker: &gc::Marker) {
        self.size.mark(marker);
    }
}
