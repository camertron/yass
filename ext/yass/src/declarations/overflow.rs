use magnus::{Ruby, value::Id};
use style::values::computed::Overflow;

pub fn overflow_to_id(overflow: &Overflow, ruby: &Ruby) -> Id {
    match overflow {
        Overflow::Visible => ruby.intern("visible"),
        Overflow::Hidden => ruby.intern("hidden"),
        Overflow::Scroll => ruby.intern("scroll"),
        Overflow::Auto => ruby.intern("auto"),
        Overflow::Clip => ruby.intern("clip"),
    }
}
