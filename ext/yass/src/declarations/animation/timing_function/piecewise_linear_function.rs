use magnus::{DataTypeFunctions, Error, IntoValue, RArray, Ruby, TypedData, gc, typed_data};
use style::piecewise_linear::{PiecewiseLinearFunction, PiecewiseLinearFunctionEntry};

use crate::cached_value_list::CachedValueList;

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Animation::TimingFunction::PiecewiseLinearFunction", mark)]
pub struct YPiecewiseLinearFunction {
    values: CachedValueList<PiecewiseLinearFunctionEntry>
}

impl YPiecewiseLinearFunction {
    pub fn new(linear_function: PiecewiseLinearFunction) -> Self {
        Self {
            values: CachedValueList::new(linear_function.iter().map(|value| value.clone()).collect(), |value, _ctx, ruby| {
                YPiecewiseLinearFunctionEntry::new(*value).into_value_with(ruby)
            })
        }
    }

    pub fn values(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        rb_self.values.to_a(ruby)
    }
}

impl DataTypeFunctions for YPiecewiseLinearFunction {
    fn mark(&self, marker: &gc::Marker) {
        self.values.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::Declarations::Animation::TimingFunction::PiecewiseLinearFunctionEntry")]
pub struct YPiecewiseLinearFunctionEntry {
    entry: PiecewiseLinearFunctionEntry
}

impl YPiecewiseLinearFunctionEntry {
    pub fn new(entry: PiecewiseLinearFunctionEntry) -> Self {
        Self { entry }
    }

    pub fn x(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> f32 {
        rb_self.entry.x
    }

    pub fn y(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> f32 {
        rb_self.entry.y
    }
}
