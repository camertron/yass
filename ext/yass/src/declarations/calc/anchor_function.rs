use magnus::{DataTypeFunctions, Error, IntoValue, RArray, Ruby, TypedData, Value, gc, typed_data, value::Id};
use style::values::{generics::{Optional, position::{AnchorSideKeyword, GenericAnchorFunction, GenericAnchorSide}}, specified::calc::CalcNode};

use crate::{cached_value::CachedValue, declarations::calc::YCalc};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Calc::AnchorFunction", mark)]
pub struct YAnchorFunction {
    target_element: CachedValue<String>,
    side: CachedValue<GenericAnchorSide<Box<CalcNode>>>,
    fallback: CachedValue<Optional<Box<CalcNode>>>
}

impl YAnchorFunction {
    pub fn new(anchor_function: GenericAnchorFunction<Box<CalcNode>, Box<CalcNode>>) -> Self {
        Self {
            target_element: CachedValue::new(anchor_function.target_element.value.0.to_string(), |el, ruby| {
                ruby.str_new(el).into_value_with(ruby)
            }),

            side: CachedValue::new(anchor_function.side, |side, ruby| {
                match side {
                    GenericAnchorSide::Percentage(p) => {
                        YCalc::make_node(p, ruby)
                    },

                    GenericAnchorSide::Keyword(k) => {
                        YAnchorSideKeyword::new(k.clone()).into_value_with(ruby)
                    }
                }
            }),

            fallback: CachedValue::new(anchor_function.fallback, |fallback, ruby| {
                match fallback {
                    Optional::Some(fb) => YCalc::make_node(fb, ruby),
                    Optional::None => ruby.qnil().into_value_with(ruby)
                }
            })
        }
    }

    pub fn target_element(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.target_element.get(ruby)
    }

    pub fn side(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.side.get(ruby)
    }

    pub fn fallback(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.fallback.get(ruby)
    }

    pub fn children(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        let result = ruby.ary_new_capa(3);
        result.push(rb_self.target_element.get(ruby))?;
        result.push(rb_self.side.get(ruby))?;
        result.push(rb_self.fallback.get(ruby))?;
        Ok(result)
    }
}

impl DataTypeFunctions for YAnchorFunction {
    fn mark(&self, marker: &gc::Marker) {
        self.target_element.mark(marker);
        self.side.mark(marker);
        self.fallback.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::Declarations::Calc::AnchorSideKeyword")]
pub struct YAnchorSideKeyword {
    anchor_side_keyword: AnchorSideKeyword
}

impl YAnchorSideKeyword {
    pub fn new(anchor_side_keyword: AnchorSideKeyword) -> Self {
        Self { anchor_side_keyword }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.anchor_side_keyword {
            AnchorSideKeyword::Inside => ruby.intern("inside"),
            AnchorSideKeyword::Outside => ruby.intern("outside"),
            AnchorSideKeyword::Top => ruby.intern("top"),
            AnchorSideKeyword::Left => ruby.intern("left"),
            AnchorSideKeyword::Right => ruby.intern("right"),
            AnchorSideKeyword::Bottom => ruby.intern("bottom"),
            AnchorSideKeyword::Start => ruby.intern("start"),
            AnchorSideKeyword::End => ruby.intern("end"),
            AnchorSideKeyword::SelfStart => ruby.intern("self_start"),
            AnchorSideKeyword::SelfEnd => ruby.intern("self_end"),
            AnchorSideKeyword::Center => ruby.intern("center"),
        }
    }
}
