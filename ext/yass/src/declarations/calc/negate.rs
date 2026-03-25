use magnus::{DataTypeFunctions, Error, RArray, Ruby, TypedData, gc, typed_data};
use style::values::specified::calc::CalcNode;

use crate::{cached_value::CachedValue, declarations::calc::YCalc};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Calc::Negate", mark)]
pub struct YNegate {
    child: CachedValue<CalcNode>
}

impl YNegate {
    pub fn new(child: CalcNode) -> Self {
        Self {
            child: CachedValue::new(child, |calc_node, ruby| {
                YCalc::make_node(calc_node, ruby)
            })
        }
    }

    pub fn children(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        let result = ruby.ary_new_capa(1);
        result.push(rb_self.child.get(ruby))?;
        Ok(result)
    }
}

impl DataTypeFunctions for YNegate {
    fn mark(&self, marker: &gc::Marker) {
        self.child.mark(marker);
    }
}
