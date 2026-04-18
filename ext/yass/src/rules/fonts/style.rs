use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc, typed_data};
use style::values::{specified::Angle};
use style::font_face::FontStyle;

use crate::{cached_value::CachedValue, declarations::angle::YAngle};

pub fn font_style_to_value(style: &FontStyle, ruby: &Ruby) -> Value {
    match style {
        FontStyle::Italic => YFontStyleItalic::new().into_value_with(ruby),
        FontStyle::Oblique(angle1, angle2) => {
            YFontStyleOblique::new(angle1.clone(), angle2.clone()).into_value_with(ruby)
        },
    }
}

#[magnus::wrap(class = "Yass::FontStyle::Italic")]
pub struct YFontStyleItalic {}

impl YFontStyleItalic {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::FontStyle::Oblique", mark)]
pub struct YFontStyleOblique {
    angle1: CachedValue<Angle>,
    angle2: CachedValue<Angle>,
}

impl YFontStyleOblique {
    pub fn new(angle1: Angle, angle2: Angle) -> Self {
        Self {
            angle1: CachedValue::new(angle1, |angle, ruby| {
                YAngle::new(*angle).into_value_with(ruby)
            }),

            angle2: CachedValue::new(angle2, |angle, ruby| {
                YAngle::new(*angle).into_value_with(ruby)
            })
        }
    }

    pub fn angle1(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.angle1.get(ruby)
    }

    pub fn angle2(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.angle2.get(ruby)
    }
}

impl DataTypeFunctions for YFontStyleOblique {
    fn mark(&self, marker: &gc::Marker) {
        self.angle1.mark(marker);
        self.angle2.mark(marker);
    }
}
