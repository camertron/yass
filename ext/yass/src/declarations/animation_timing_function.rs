use magnus::{DataTypeFunctions, Error, IntoValue, RArray, Ruby, TypedData, gc, typed_data};
use style::{piecewise_linear::PiecewiseLinearFunction, properties::longhands::animation_timing_function::SpecifiedValue, values::{generics::easing::TimingFunction, specified::{Integer, Number}}};

use crate::{cached_value_list::CachedValueList, declarations::{animation::timing_function::{cubic_bezier::YCubicBezier, keyword::YKeyword, piecewise_linear_function::YPiecewiseLinearFunction, steps::YSteps}}};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::AnimationTimingFunction", mark)]
pub struct YAnimationTimingFunction {
    values: CachedValueList<TimingFunction<Integer, Number, PiecewiseLinearFunction>>
}

impl YAnimationTimingFunction {
    pub fn new(specified_value: SpecifiedValue) -> Self {
        Self {
            values: CachedValueList::new(specified_value.0.to_vec(), |value, _ctx, ruby| {
                match value {
                    TimingFunction::Keyword(timing_keyword) => {
                        YKeyword::new(timing_keyword.clone()).into_value_with(ruby)
                    },

                    TimingFunction::CubicBezier { x1, y1, x2, y2 } => {
                        YCubicBezier::new(x1.clone(), y1.clone(), x2.clone(), y2.clone()).into_value_with(ruby)
                    },

                    TimingFunction::Steps(steps, step_position) => {
                        YSteps::new(steps.value(), *step_position).into_value_with(ruby)
                    },

                    TimingFunction::LinearFunction(linear_function) => {
                        YPiecewiseLinearFunction::new(linear_function.clone()).into_value_with(ruby)
                    },
                }
            })
        }
    }

    pub fn values(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        rb_self.values.to_a(ruby)
    }
}

impl DataTypeFunctions for YAnimationTimingFunction {
    fn mark(&self, marker: &gc::Marker) {
        self.values.mark(marker);
    }
}
