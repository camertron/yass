use magnus::{Ruby, typed_data, value::Id};
use style::computed_values::text_rendering::T;

#[magnus::wrap(class = "Yass::Declarations::TextRendering")]
pub struct YTextRendering {
    specified_value: T
}

impl YTextRendering {
    pub fn new(specified_value: T) -> Self {
        Self { specified_value }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.specified_value {
            T::Auto => ruby.intern("auto"),
            T::Optimizespeed => ruby.intern("optimize_speed"),
            T::Optimizelegibility => ruby.intern("optimize_legibility"),
            T::Geometricprecision => ruby.intern("geometric_precision"),
        }
    }
}
