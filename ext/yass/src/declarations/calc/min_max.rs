use magnus::{DataTypeFunctions, Error, RArray, Ruby, TypedData, gc, typed_data, value::Id};
use style::{OwnedSlice, values::{generics::calc::MinMaxOp, specified::calc::CalcNode}};

use crate::{declarations::calc::YCalc, value_list::ValueList};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Calc::MinMax", mark)]
pub struct YMinMax {
    op: MinMaxOp,
    children: ValueList<CalcNode>
}

impl YMinMax {
    pub fn new(children: &OwnedSlice<CalcNode>, op: MinMaxOp) -> Self {
        Self {
            children: ValueList::new(children.to_vec(), |calc_node, _ctx, ruby| {
                YCalc::make_node(calc_node, ruby)
            }),

            op
        }
    }

    pub fn children(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        rb_self.children.to_a(ruby)
    }

    pub fn op(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.op {
            MinMaxOp::Min => ruby.intern("min"),
            MinMaxOp::Max => ruby.intern("max"),
        }
    }
}

impl DataTypeFunctions for YMinMax {
    fn mark(&self, marker: &gc::Marker) {
        self.children.mark(marker);
    }
}
