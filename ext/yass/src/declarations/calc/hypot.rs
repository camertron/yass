use magnus::{DataTypeFunctions, Error, RArray, Ruby, TypedData, gc, typed_data};
use style::{OwnedSlice, values::specified::calc::CalcNode};

use crate::{declarations::calc::YCalc, cached_value_list::CachedValueList};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Calc::Hypot", mark)]
pub struct YHypot {
    children: CachedValueList<CalcNode>
}

impl YHypot {
    pub fn new(children: &OwnedSlice<CalcNode>) -> Self {
        Self {
            children: CachedValueList::new(children.to_vec(), |calc_node, _ctx, ruby| {
                YCalc::make_node(calc_node, ruby)
            })
        }
    }

    pub fn children(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        rb_self.children.to_a(ruby)
    }
}

impl DataTypeFunctions for YHypot {
    fn mark(&self, marker: &gc::Marker) {
        self.children.mark(marker);
    }
}
