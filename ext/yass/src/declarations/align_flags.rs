use magnus::{Ruby, value::Id};
use style::values::specified::align::AlignFlags;

pub fn to_id(flags: AlignFlags, ruby: &Ruby) -> Option<Id> {
    match flags {
        AlignFlags::AUTO => Some(ruby.intern("auto")),
        AlignFlags::NORMAL => Some(ruby.intern("normal")),
        AlignFlags::START => Some(ruby.intern("start")),
        AlignFlags::END => Some(ruby.intern("end")),
        AlignFlags::FLEX_START => Some(ruby.intern("flex_start")),
        AlignFlags::FLEX_END => Some(ruby.intern("flex_end")),
        AlignFlags::CENTER => Some(ruby.intern("center")),
        AlignFlags::LEFT => Some(ruby.intern("left")),
        AlignFlags::RIGHT => Some(ruby.intern("right")),
        AlignFlags::BASELINE => Some(ruby.intern("baseline")),
        AlignFlags::LAST_BASELINE => Some(ruby.intern("last_baseline")),
        AlignFlags::STRETCH => Some(ruby.intern("stretch")),
        AlignFlags::SELF_START => Some(ruby.intern("self_start")),
        AlignFlags::SELF_END => Some(ruby.intern("self_end")),
        AlignFlags::SPACE_BETWEEN => Some(ruby.intern("space_between")),
        AlignFlags::SPACE_AROUND => Some(ruby.intern("space_around")),
        AlignFlags::SPACE_EVENLY => Some(ruby.intern("space_evenly")),
        AlignFlags::ANCHOR_CENTER => Some(ruby.intern("anchor_center")),
        _ => None
    }
}
