use magnus::{Ruby, typed_data, value::Id};
use style::values::computed::FontSynthesis;

#[magnus::wrap(class = "Yass::Declarations::FontSynthesisWeight")]
pub struct YFontSynthesisWeight {
    font_synthesis: FontSynthesis
}

impl YFontSynthesisWeight {
    pub fn new(font_synthesis: FontSynthesis) -> Self {
        Self { font_synthesis }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.font_synthesis {
            FontSynthesis::Auto => ruby.intern("auto"),
            FontSynthesis::None => ruby.intern("none"),
        }
    }
}
