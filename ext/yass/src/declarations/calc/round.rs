use magnus::{DataTypeFunctions, Error, RArray, Ruby, TypedData, Value, gc, typed_data, value::Id};
use style::values::{generics::calc::RoundingStrategy, specified::calc::CalcNode};

use crate::{cached_value::CachedValue, declarations::calc::YCalc};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Calc::Round", mark)]
pub struct YRound {
    rounding_strategy: RoundingStrategy,
    value: CachedValue<CalcNode>,
    step: CachedValue<CalcNode>
}

impl YRound {
    pub fn new(rounding_strategy: RoundingStrategy, value: CalcNode, step: CalcNode) -> Self {
        Self {
            rounding_strategy,

            value: CachedValue::new(value, |calc_node, ruby| {
                YCalc::make_node(calc_node, ruby)
            }),

            step: CachedValue::new(step, |calc_node, ruby| {
                YCalc::make_node(calc_node, ruby)
            })
        }
    }

    pub fn children(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        let result = ruby.ary_new_capa(2);
        result.push(rb_self.value.get(ruby))?;
        result.push(rb_self.step.get(ruby))?;
        Ok(result)
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.value.get(ruby)
    }

    pub fn step(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.step.get(ruby)
    }

    pub fn rounding_strategy(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.rounding_strategy {
            RoundingStrategy::Nearest => ruby.intern("nearest"),
            RoundingStrategy::Up => ruby.intern("up"),
            RoundingStrategy::Down => ruby.intern("down"),
            RoundingStrategy::ToZero => ruby.intern("tozero"),
        }
    }
}

impl DataTypeFunctions for YRound {
    fn mark(&self, marker: &gc::Marker) {
        self.value.mark(marker);
        self.step.mark(marker);
    }
}
