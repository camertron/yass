use magnus::{DataTypeFunctions, Error, IntoValue, RArray, Ruby, TypedData, gc, typed_data};
use style::{properties::longhands::animation_timeline::SpecifiedValue, values::{generics::{animation::AnimationTimeline}, specified::LengthPercentage}};
use style_traits::ToCss;

use crate::{cached_value_list::CachedValueList, declarations::{animation::{auto::YAuto, scroll_function::YScrollFunction, timeline_name::YTimelineName, view_function::YViewFunction}}};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::AnimationTimeline", mark)]
pub struct YAnimationTimeline {
    values: CachedValueList<AnimationTimeline<LengthPercentage>>
}

impl YAnimationTimeline {
    pub fn new(specified_value: SpecifiedValue) -> Self {
        Self {
            values: CachedValueList::new(specified_value.0.to_vec(), |value, _ctx, ruby| {
                match value {
                    AnimationTimeline::Auto => YAuto::new().into_value_with(ruby),
                    AnimationTimeline::Scroll(scroll_fn) => {
                        YScrollFunction::new(scroll_fn.clone()).into_value_with(ruby)
                    },
                    AnimationTimeline::Timeline(timeline_name) => {
                        YTimelineName::new(timeline_name.to_css_string()).into_value_with(ruby)
                    },
                    AnimationTimeline::View(generic_view_function) => {
                        YViewFunction::new(generic_view_function.clone()).into_value_with(ruby)
                    },
                }
            })
        }
    }

    pub fn values(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        rb_self.values.to_a(ruby)
    }
}

impl DataTypeFunctions for YAnimationTimeline {
    fn mark(&self, marker: &gc::Marker) {
        self.values.mark(marker);
    }
}
