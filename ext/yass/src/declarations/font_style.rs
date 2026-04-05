use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc, typed_data};
use style::values::{generics::font::FontStyle as GenericFontStyle, specified::{Angle, FontStyle, font::SpecifiedFontStyle}};

use crate::{cached_value::CachedValue, declarations::angle::YAngle};

fn make_font_style(font_style: &FontStyle, ruby: &Ruby) -> Value {
    match font_style {
        FontStyle::Specified(specified) => make_specified_font_style(specified, ruby),
        FontStyle::System(_) => YFontStyleSystem::new().into_value_with(ruby),
    }
}

fn make_specified_font_style(font_style: &SpecifiedFontStyle, ruby: &Ruby) -> Value {
    match font_style {
        GenericFontStyle::Italic => YFontStyleItalic::new().into_value_with(ruby),
        GenericFontStyle::Oblique(angle) => {
            if angle.degrees() == 0.0 && !angle.was_calc() {
                YFontStyleNormal::new().into_value_with(ruby)
            } else {
                YFontStyleOblique::new(*angle).into_value_with(ruby)
            }
        },
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::FontStyle", mark)]
pub struct YFontStyle {
    font_style: CachedValue<FontStyle>
}

impl YFontStyle {
    pub fn new(font_style: FontStyle) -> Self {
        Self {
            font_style: CachedValue::new(font_style, make_font_style),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.font_style.get(ruby)
    }
}

impl DataTypeFunctions for YFontStyle {
    fn mark(&self, marker: &gc::Marker) {
        self.font_style.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::Declarations::FontStyle::Normal")]
pub struct YFontStyleNormal {}

impl YFontStyleNormal {
    pub fn new() -> Self {
        Self {}
    }
}

#[magnus::wrap(class = "Yass::Declarations::FontStyle::Italic")]
pub struct YFontStyleItalic {}

impl YFontStyleItalic {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::FontStyle::Oblique", mark)]
pub struct YFontStyleOblique {
    angle: CachedValue<Angle>,
}

impl YFontStyleOblique {
    pub fn new(angle: Angle) -> Self {
        Self {
            angle: CachedValue::new(angle, |value, ruby| {
                YAngle::new(*value).into_value_with(ruby)
            }),
        }
    }

    pub fn angle(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.angle.get(ruby)
    }
}

impl DataTypeFunctions for YFontStyleOblique {
    fn mark(&self, marker: &gc::Marker) {
        self.angle.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::Declarations::FontStyle::System")]
pub struct YFontStyleSystem {}

impl YFontStyleSystem {
    pub fn new() -> Self {
        Self {}
    }
}
