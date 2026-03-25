use magnus::{DataTypeFunctions, Error, IntoValue, RArray, RString, Ruby, TypedData, Value, gc, typed_data};
use style::{properties::longhands::animation_timeline::SpecifiedValue, values::{computed::ScrollAxis, generics::{animation::{AnimationTimeline, ViewFunction, ViewTimelineInset}, length::LengthPercentageOrAuto}, specified::{LengthPercentage, animation::{ScrollFunction, Scroller}}}};
use style_traits::ToCss;

use crate::{cached_value::CachedValue, declarations::{calc::YCalc, length::YLength, percentage::YPercentage}, value_list::ValueList};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::AnimationTimeline", mark)]
pub struct YAnimationTimeline {
    cached_timeline_values: ValueList<AnimationTimeline<LengthPercentage>>
}

impl YAnimationTimeline {
    pub fn new(specified_value: SpecifiedValue) -> Self {
        Self {
            cached_timeline_values: ValueList::new(specified_value.0.to_vec(), |value, _ctx, ruby| {
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
        rb_self.cached_timeline_values.to_a(ruby)
    }
}

impl DataTypeFunctions for YAnimationTimeline {
    fn mark(&self, marker: &gc::Marker) {
        self.cached_timeline_values.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::Declarations::AnimationTimeline::Auto")]
pub struct YAuto { }

impl YAuto {
    pub fn new() -> Self {
        Self { }
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::ScrollFunction", mark)]
pub struct YScrollFunction {
    scroller: CachedValue<Scroller>,
    scroll_axis: CachedValue<ScrollAxis>
}

impl YScrollFunction {
    pub fn new(scroll_function: ScrollFunction) -> Self {
        YScrollFunction {
            scroller: CachedValue::new(scroll_function.scroller, |scroller, ruby| {
                YScroller::new(*scroller).into_value_with(ruby)
            }),

            scroll_axis: CachedValue::new(scroll_function.axis, |axis, ruby| {
                YScrollAxis::new(*axis).into_value_with(ruby)
            })
        }
    }

    pub fn scroller(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.scroller.get(ruby)
    }

    pub fn scroll_axis(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.scroll_axis.get(ruby)
    }
}

impl DataTypeFunctions for YScrollFunction {
    fn mark(&self, marker: &gc::Marker) {
        self.scroller.mark(marker);
        self.scroll_axis.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::Declarations::Scroller")]
pub struct YScroller {
    scroller: Scroller
}

impl YScroller {
    pub fn new(scroller: Scroller) -> Self {
        Self { scroller }
    }
}

#[magnus::wrap(class = "Yass::Declarations::ScrollAxis")]
pub struct YScrollAxis {
    scroll_axis: ScrollAxis
}

impl YScrollAxis {
    pub fn new(scroll_axis: ScrollAxis) -> Self {
        Self { scroll_axis }
    }
}

#[magnus::wrap(class = "Yass::Declarations::AnimationTimeline::TimelineName")]
pub struct YTimelineName {
    timeline_name: String
}

impl YTimelineName {
    pub fn new(timeline_name: String) -> Self {
        Self { timeline_name }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> RString {
        ruby.str_new(&rb_self.timeline_name)
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::ViewFunction", mark)]
pub struct YViewFunction {
    scroll_axis: CachedValue<ScrollAxis>,
    inset: CachedValue<ViewTimelineInset<LengthPercentage>>
}

impl YViewFunction {
    pub fn new(view_function: ViewFunction<LengthPercentage>) -> Self {
        Self {
            scroll_axis: CachedValue::new(view_function.axis, |axis, ruby| {
                YScrollAxis::new(*axis).into_value_with(ruby)
            }),

            inset: CachedValue::new(view_function.inset, |inset, ruby| {
                YInset::new(inset.clone()).into_value_with(ruby)
            })
        }
    }
}

impl DataTypeFunctions for YViewFunction {
    fn mark(&self, marker: &gc::Marker) {
        self.scroll_axis.mark(marker);
        self.inset.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::AnimationTimeline::Inset", mark)]
pub struct YInset {
    start: CachedValue<LengthPercentageOrAuto<LengthPercentage>>,
    end: CachedValue<LengthPercentageOrAuto<LengthPercentage>>
}

impl YInset {
    pub fn new(inset: ViewTimelineInset<LengthPercentage>) -> Self {
        Self {
            start: CachedValue::new(inset.start, |start, ruby| {
                match start {
                    LengthPercentageOrAuto::Auto => YLengthAuto::new().into_value_with(ruby),
                    LengthPercentageOrAuto::LengthPercentage(length) => {
                        YLengthPercentage::make(length.clone(), ruby)
                    }
                }
            }),

            end: CachedValue::new(inset.end, |end, ruby| {
                match end {
                    LengthPercentageOrAuto::Auto => YLengthAuto::new().into_value_with(ruby),
                    LengthPercentageOrAuto::LengthPercentage(length) => {
                        YLengthPercentage::make(length.clone(), ruby)
                    }
                }
            })
        }
    }

    pub fn start(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.start.get(ruby)
    }
}

impl DataTypeFunctions for YInset {
    fn mark(&self, marker: &gc::Marker) {
        self.start.mark(marker);
        self.end.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::Declarations::Length::Auto")]
pub struct YLengthAuto { }

impl YLengthAuto {
    pub fn new() -> Self {
        Self { }
    }
}

pub struct YLengthPercentage {}
impl YLengthPercentage {
    pub fn make(length_percentage: LengthPercentage, ruby: &Ruby) -> Value {
        match length_percentage {
            LengthPercentage::Length(length) => {
                YLength::make(length, ruby)
            },
            LengthPercentage::Percentage(percentage) => {
                YPercentage::new(percentage.0).into_value_with(ruby)
            },
            LengthPercentage::Calc(calc) => {
                YCalc::new(calc).into_value_with(ruby)
            }
        }
    }
}
