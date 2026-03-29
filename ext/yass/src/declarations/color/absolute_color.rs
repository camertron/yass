use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc, typed_data, value::Id};
use style::color::{AbsoluteColor, ColorComponents};

use crate::{cached_value::CachedValue, declarations::{color::color_components::YColorComponents}};

use super::{color_space_to_id};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Color::AbsoluteColor", mark)]
pub struct YAbsoluteColor {
    absolute_color: AbsoluteColor,
    components: CachedValue<ColorComponents>,
    alpha: Option<f32>,
}

impl YAbsoluteColor {
    pub fn new(absolute_color: AbsoluteColor) -> Self {
        Self {
            components: CachedValue::new(absolute_color.components, |components, ruby| {
                YColorComponents::new(*components).into_value_with(ruby)
            }),

            alpha: absolute_color.alpha(),
            absolute_color,
        }
    }

    pub fn components(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.components.get(ruby)
    }

    pub fn alpha(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Option<f32> {
        rb_self.alpha
    }

    pub fn color_space(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        color_space_to_id(rb_self.absolute_color.color_space, ruby)
    }

    pub fn legacy_syntax(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.absolute_color.is_legacy_syntax()
    }

    pub fn transparent(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.absolute_color.is_transparent()
    }
}

impl DataTypeFunctions for YAbsoluteColor {
    fn mark(&self, marker: &gc::Marker) {
        self.components.mark(marker);
    }
}
