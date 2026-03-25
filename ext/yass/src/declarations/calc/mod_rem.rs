use magnus::{DataTypeFunctions, Error, RArray, Ruby, TypedData, Value, gc, typed_data, value::Id};
use style::values::{generics::calc::ModRemOp, specified::calc::CalcNode};

use crate::{cached_value::CachedValue, declarations::calc::YCalc};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Calc::ModRem", mark)]
pub struct YModRem {
    dividend: CachedValue<CalcNode>,
    divisor: CachedValue<CalcNode>,
    op: ModRemOp
}

impl YModRem {
    pub fn new(dividend: CalcNode, divisor: CalcNode, op: ModRemOp) -> Self {
        Self {
            dividend: CachedValue::new(dividend, |calc_node, ruby| {
                YCalc::make_node(calc_node, ruby)
            }),

            divisor: CachedValue::new(divisor, |calc_node, ruby| {
                YCalc::make_node(calc_node, ruby)
            }),

            op
        }
    }

    pub fn children(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        let result = ruby.ary_new_capa(3);
        result.push(rb_self.dividend.get(ruby))?;
        result.push(rb_self.divisor.get(ruby))?;
        Ok(result)
    }

    pub fn dividend(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.dividend.get(ruby)
    }

    pub fn divisor(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.divisor.get(ruby)
    }

    pub fn op(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.op {
            ModRemOp::Mod => ruby.intern("mod"),
            ModRemOp::Rem => ruby.intern("rem"),
        }
    }
}

impl DataTypeFunctions for YModRem {
    fn mark(&self, marker: &gc::Marker) {
        self.dividend.mark(marker);
        self.divisor.mark(marker);
    }
}
