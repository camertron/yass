use magnus::{DataTypeFunctions, Error, RArray, Ruby, TypedData, gc, typed_data};
use style::{OwnedSlice, values::specified::calc::CalcNode};

use crate::{declarations::calc::YCalc, value_list::ValueList};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Calc::Sum", mark)]
pub struct YSum {
    children: ValueList<CalcNode>
}

impl YSum {
    pub fn new(children: &OwnedSlice<CalcNode>) -> Self {
        Self {
            children: ValueList::new(children.to_vec(), |calc_node, _ctx, ruby| {
                YCalc::make_node(calc_node, ruby)
            })
        }
    }

    pub fn children(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        rb_self.children.to_a(ruby)
    }
}

impl DataTypeFunctions for YSum {
    fn mark(&self, marker: &gc::Marker) {
        self.children.mark(marker);
    }
}
