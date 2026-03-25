use magnus::{DataTypeFunctions, Error, RArray, Ruby, TypedData, Value, gc, typed_data};
use style::values::specified::calc::CalcNode;

use crate::{cached_value::CachedValue, declarations::calc::YCalc};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Calc::Clamp", mark)]
pub struct YClamp {
    min: CachedValue<CalcNode>,
    center: CachedValue<CalcNode>,
    max: CachedValue<CalcNode>
}

impl YClamp {
    pub fn new(min: CalcNode, center: CalcNode, max: CalcNode) -> Self {
        Self {
            min: CachedValue::new(min, |calc_node, ruby| {
                YCalc::make_node(calc_node, ruby)
            }),

            center: CachedValue::new(center, |calc_node, ruby| {
                YCalc::make_node(calc_node, ruby)
            }),

            max: CachedValue::new(max, |calc_node, ruby| {
                YCalc::make_node(calc_node, ruby)
            })
        }
    }

    pub fn children(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        let result = ruby.ary_new_capa(3);
        result.push(rb_self.min.get(ruby))?;
        result.push(rb_self.center.get(ruby))?;
        result.push(rb_self.max.get(ruby))?;
        Ok(result)
    }

    pub fn min(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.min.get(ruby)
    }

    pub fn center(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.center.get(ruby)
    }

    pub fn max(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.max.get(ruby)
    }
}

impl DataTypeFunctions for YClamp {
    fn mark(&self, marker: &gc::Marker) {
        self.min.mark(marker);
        self.center.mark(marker);
        self.max.mark(marker);
    }
}
