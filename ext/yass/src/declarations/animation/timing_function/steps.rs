use magnus::{Ruby, typed_data, value::Id};
use style::values::generics::easing::StepPosition;

#[magnus::wrap(class = "Yass::Declarations::Animation::TimingFunction::Steps")]
pub struct YSteps {
    count: i32,
    position: StepPosition
}

impl YSteps {
    pub fn new(count: i32, position: StepPosition) -> Self {
        Self { count, position }
    }

    pub fn count(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> i32 {
        let _ = ruby;
        rb_self.count
    }

    pub fn position(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.position {
            StepPosition::JumpStart => ruby.intern("jump_start"),
            StepPosition::JumpEnd => ruby.intern("jump_end"),
            StepPosition::JumpNone => ruby.intern("jump_none"),
            StepPosition::JumpBoth => ruby.intern("jump_both"),
            StepPosition::Start => ruby.intern("start"),
            StepPosition::End => ruby.intern("end"),
        }
    }
}
