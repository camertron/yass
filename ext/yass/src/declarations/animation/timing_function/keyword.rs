use magnus::{Ruby, typed_data, value::Id};
use style::values::generics::easing::TimingKeyword;

#[magnus::wrap(class = "Yass::Declarations::Animation::TimingFunction::Keyword")]
pub struct YKeyword {
    keyword: TimingKeyword
}

impl YKeyword {
    pub fn new(keyword: TimingKeyword) -> Self {
        Self { keyword }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.keyword {
            TimingKeyword::Linear => ruby.intern("linear"),
            TimingKeyword::Ease => ruby.intern("ease"),
            TimingKeyword::EaseIn => ruby.intern("ease_in"),
            TimingKeyword::EaseOut => ruby.intern("ease_out"),
            TimingKeyword::EaseInOut => ruby.intern("ease_in_out"),
        }
    }
}
