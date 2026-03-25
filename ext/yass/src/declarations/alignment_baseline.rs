use magnus::{Ruby, typed_data, value::Id};
use style::values::computed::AlignmentBaseline;

#[magnus::wrap(class = "Yass::Declarations::AlignmentBaseline")]
pub struct YAlignmentBaseline {
    alignment_baseline: AlignmentBaseline
}

impl YAlignmentBaseline {
    pub fn new(alignment_baseline: AlignmentBaseline) -> Self {
        Self { alignment_baseline }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.alignment_baseline {
            AlignmentBaseline::Baseline => ruby.intern("baseline"),
            AlignmentBaseline::TextBottom => ruby.intern("text_bottom"),
            AlignmentBaseline::Middle => ruby.intern("middle"),
            AlignmentBaseline::TextTop => ruby.intern("text_top"),
            AlignmentBaseline::Alphabetic => ruby.intern("alphabetic"),
            AlignmentBaseline::Ideographic => ruby.intern("ideographic"),
            AlignmentBaseline::Central => ruby.intern("central"),
            AlignmentBaseline::Mathematical => ruby.intern("mathematical"),
            AlignmentBaseline::Hanging => ruby.intern("hanging"),
        }
    }
}
