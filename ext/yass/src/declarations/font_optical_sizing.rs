use magnus::{Ruby, typed_data, value::Id};
use style::computed_values::font_optical_sizing::T;

#[magnus::wrap(class = "Yass::Declarations::FontOpticalSizing")]
pub struct YFontOpticalSizing {
    specified_value: T
}

impl YFontOpticalSizing {
    pub fn new(specified_value: T) -> Self {
        Self { specified_value }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.specified_value {
            T::Auto => ruby.intern("auto"),
            T::None => ruby.intern("none"),
        }
    }
}
