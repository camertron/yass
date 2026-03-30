use magnus::{Ruby, typed_data, value::Id};
use style::values::computed::BaselineSource;

#[magnus::wrap(class = "Yass::Declarations::BaselineSource")]
pub struct YBaselineSource {
    baseline_source: BaselineSource
}

impl YBaselineSource {
    pub fn new(baseline_source: BaselineSource) -> Self {
        Self { baseline_source }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.baseline_source {
            BaselineSource::Auto => ruby.intern("auto"),
            BaselineSource::First => ruby.intern("first"),
            BaselineSource::Last => ruby.intern("last"),
        }
    }
}
