use magnus::{Ruby, typed_data, value::Id};
use style::computed_values::font_variant_caps::T;

#[magnus::wrap(class = "Yass::Declarations::FontVariantCaps")]
pub struct YFontVariantCaps {
    specified_value: T
}

impl YFontVariantCaps {
    pub fn new(specified_value: T) -> Self {
        Self { specified_value }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.specified_value {
            T::Normal => ruby.intern("normal"),
            T::SmallCaps => ruby.intern("small_caps"),
        }
    }
}
