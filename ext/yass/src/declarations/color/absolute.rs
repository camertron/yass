use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc, typed_data};
use style::{
    color::AbsoluteColor,
    values::specified::color::Absolute as SpecifiedAbsolute,
};

use crate::{cached_value::CachedValue, declarations::color::absolute_color::YAbsoluteColor};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Color::Absolute", mark)]
pub struct YAbsolute {
    color: CachedValue<AbsoluteColor>,
    authored: CachedValue<Option<String>>,
}

impl YAbsolute {
    pub fn new(absolute: SpecifiedAbsolute) -> Self {
        Self {
            color: CachedValue::new(absolute.color, |absolute_color, ruby| {
                YAbsoluteColor::new(*absolute_color).into_value_with(ruby)
            }),

            authored: CachedValue::new(absolute.authored.map(|authored| authored.to_string()), |authored, ruby| {
                match authored {
                    Some(authored) => ruby.str_new(authored).into_value_with(ruby),
                    None => ruby.qnil().into_value_with(ruby),
                }
            }),
        }
    }

    pub fn color(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.color.get(ruby)
    }

    pub fn authored(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.authored.get(ruby)
    }
}

impl DataTypeFunctions for YAbsolute {
    fn mark(&self, marker: &gc::Marker) {
        self.color.mark(marker);
        self.authored.mark(marker);
    }
}
