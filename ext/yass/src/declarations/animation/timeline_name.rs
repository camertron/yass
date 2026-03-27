use magnus::{RString, Ruby, typed_data};

#[magnus::wrap(class = "Yass::Declarations::Animation::TimelineName")]
pub struct YTimelineName {
    timeline_name: String
}

impl YTimelineName {
    pub fn new(timeline_name: String) -> Self {
        Self { timeline_name }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> RString {
        ruby.str_from_slice(rb_self.timeline_name.as_bytes())
    }
}
