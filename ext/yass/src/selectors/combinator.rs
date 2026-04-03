use magnus::{Ruby, Value, value::{Id, ReprValue}};
use selectors::parser::Combinator;

#[magnus::wrap(class = "Yass::Selector::Combinator")]
pub struct YCombinator {  // lol
    pub combinator: Combinator
}

impl YCombinator {
    pub fn new(combinator: Combinator) -> Self {
        YCombinator { combinator }
    }

    pub fn is_ancestor(&self, ruby: &Ruby) -> Value {
        if self.combinator.is_ancestor() {
            ruby.qtrue().as_value()
        } else {
            ruby.qfalse().as_value()
        }
    }

    pub fn is_pseudo_element(&self, ruby: &Ruby) -> Value {
        if self.combinator.is_pseudo_element() {
            ruby.qtrue().as_value()
        } else {
            ruby.qfalse().as_value()
        }
    }

    pub fn is_sibling(&self, ruby: &Ruby) -> Value {
        if self.combinator.is_sibling() {
            ruby.qtrue().as_value()
        } else {
            ruby.qfalse().as_value()
        }
    }

    pub fn kind(&self, ruby: &Ruby) -> Id {
        match self.combinator {
            Combinator::Child => ruby.intern("child"),
            Combinator::Descendant => ruby.intern("descendant"),
            Combinator::NextSibling => ruby.intern("next_sibling"),
            Combinator::LaterSibling => ruby.intern("later_sibling"),
            Combinator::PseudoElement => ruby.intern("pseudo_element"),
            Combinator::SlotAssignment => ruby.intern("slot_assignment"),
            Combinator::Part => ruby.intern("part"),
        }
    }
}
