use magnus::{DataTypeFunctions, Ruby, TypedData, Value, gc, typed_data, value::{Id, Opaque, ReprValue}};
use selectors::parser::{RelativeSelector, RelativeSelectorMatchHint};
use style::selector_parser::SelectorImpl;

use crate::selectors::YSelector;

#[derive(TypedData)]
#[magnus(class = "Yass::RelativeSelector", mark)]
pub struct YRelativeSelector {
    selector: Opaque<typed_data::Obj<YSelector>>,
    match_hint: RelativeSelectorMatchHint
}

impl YRelativeSelector {
    pub fn new(relative_selector: RelativeSelector<SelectorImpl>, ruby: &Ruby) -> Self {
        let selector = Opaque::from(ruby.obj_wrap(YSelector::new(relative_selector.selector)));
        Self { selector, match_hint: relative_selector.match_hint }
    }

    pub fn selector(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        ruby.get_inner(rb_self.selector).as_value()
    }

    pub fn match_hint(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.match_hint {
            RelativeSelectorMatchHint::InSubtree => ruby.intern("in_subtree"),
            RelativeSelectorMatchHint::InChild => ruby.intern("in_child"),
            RelativeSelectorMatchHint::InNextSibling => ruby.intern("in_next_sibling"),
            RelativeSelectorMatchHint::InNextSiblingSubtree => ruby.intern("in_next_sibling_subtree"),
            RelativeSelectorMatchHint::InSibling => ruby.intern("in_sibling"),
            RelativeSelectorMatchHint::InSiblingSubtree => ruby.intern("in_sibling_subtree")
        }
    }
}

impl DataTypeFunctions for YRelativeSelector {
    fn mark(&self, marker: &gc::Marker) {
        marker.mark(self.selector);
    }
}
