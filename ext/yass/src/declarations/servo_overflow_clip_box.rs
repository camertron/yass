use magnus::{Ruby, typed_data, value::Id};
use style::computed_values::_servo_overflow_clip_box::T;

#[magnus::wrap(class = "Yass::Declarations::ServoOverflowClipBox")]
pub struct YServoOverflowClipBox {
    specified_value: T
}

impl YServoOverflowClipBox {
    pub fn new(specified_value: T) -> Self {
        Self { specified_value }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.specified_value {
            T::PaddingBox => ruby.intern("padding_box"),
            T::ContentBox => ruby.intern("content_box"),
        }
    }
}
