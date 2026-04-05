use magnus::{Error, Ruby, typed_data, value::Id};
use style::values::{specified::JustifyItems, specified::align::AlignFlags};

#[magnus::wrap(class = "Yass::Declarations::JustifyItems")]
pub struct YJustifyItems {
    justify_items: JustifyItems
}

impl YJustifyItems {
    pub fn new(justify_items: JustifyItems) -> Self {
        Self { justify_items }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<Id, Error> {
        match rb_self.justify_items.0.value() {
            AlignFlags::AUTO => Ok(ruby.intern("auto")),
            AlignFlags::NORMAL => Ok(ruby.intern("normal")),
            AlignFlags::START => Ok(ruby.intern("start")),
            AlignFlags::END => Ok(ruby.intern("end")),
            AlignFlags::FLEX_START => Ok(ruby.intern("flex_start")),
            AlignFlags::FLEX_END => Ok(ruby.intern("flex_end")),
            AlignFlags::CENTER => Ok(ruby.intern("center")),
            AlignFlags::LEFT => Ok(ruby.intern("left")),
            AlignFlags::RIGHT => Ok(ruby.intern("right")),
            AlignFlags::BASELINE => Ok(ruby.intern("baseline")),
            AlignFlags::LAST_BASELINE => Ok(ruby.intern("last_baseline")),
            AlignFlags::STRETCH => Ok(ruby.intern("stretch")),
            AlignFlags::SELF_START => Ok(ruby.intern("self_start")),
            AlignFlags::SELF_END => Ok(ruby.intern("self_end")),
            AlignFlags::SPACE_BETWEEN => Ok(ruby.intern("space_between")),
            AlignFlags::SPACE_AROUND => Ok(ruby.intern("space_around")),
            AlignFlags::SPACE_EVENLY => Ok(ruby.intern("space_evenly")),
            AlignFlags::ANCHOR_CENTER => Ok(ruby.intern("anchor_center")),
            _ => Err(Error::new(ruby.exception_arg_error(), "unreachable"))
        }
    }
}
