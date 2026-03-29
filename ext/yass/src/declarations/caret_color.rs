use magnus::{DataTypeFunctions, Error, IntoValue, Module, RClass, Ruby, TypedData, Value, gc, method, typed_data};
use style::values::generics::color::{GenericCaretColor, GenericColorOrAuto};
use style::values::specified::color::Color;

use crate::declarations::color::color::make_color;
use crate::{cached_value::CachedValue};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::CaretColor", mark)]
pub struct YCaretColor {
    color: CachedValue<GenericCaretColor<Color>>,
}

impl YCaretColor {
    pub fn new(caret_color: GenericCaretColor<Color>) -> Self {
        Self {
            color: CachedValue::new(caret_color, |caret_color, ruby| {
                match &caret_color.0 {
                    GenericColorOrAuto::Color(c) => make_color(c, ruby),
                    GenericColorOrAuto::Auto => ruby.qnil().into_value_with(ruby),
                }
            }),
        }
    }

    pub fn auto(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        matches!(rb_self.color.value.0, GenericColorOrAuto::Auto)
    }

    pub fn color(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.color.get(ruby)
    }
}

impl DataTypeFunctions for YCaretColor {
    fn mark(&self, marker: &gc::Marker) {
        self.color.mark(marker);
    }
}

pub fn init(ruby: &Ruby, class: &RClass) -> Result<(), Error> {
    class.define_method("auto?", method!(YCaretColor::auto, 0))?;
    class.define_method("color", method!(YCaretColor::color, 0))?;
    Ok(())
}
