use magnus::{DataTypeFunctions, Error, IntoValue, RArray, Ruby, Value, gc::Marker, typed_data, value::Id};
use style::{color::mix::ColorInterpolationMethod, values::{generics::image::{GradientCompatMode, GradientFlags, GradientItem}, specified::{Color, LengthPercentage, image::{Gradient, LineDirection}}}};

use crate::{cached_value::CachedValue, cached_value_list::CachedValueList, declarations::{color::color_interpolation_method::YColorInterpolationMethod, images::{compat_mode_to_id, gradient_length_percentage_item::make_gradient_length_percentage_item, line_direction::make_line_direction}}};

#[derive(magnus::TypedData)]
#[magnus(class = "Yass::Declarations::Image::LinearGradient", mark)]
pub struct YLinearGradient {
    direction: CachedValue<LineDirection>,
    color_interpolation_method: CachedValue<ColorInterpolationMethod>,
    items: CachedValueList<GradientItem<Color, LengthPercentage>>,
    repeating: bool,
    compat_mode: GradientCompatMode,
}

impl YLinearGradient {
    pub fn new(gradient: Gradient) -> Self {
        match gradient {
            Gradient::Linear {
                direction,
                color_interpolation_method,
                items,
                flags,
                compat_mode,
            } => Self {
                direction: CachedValue::new(direction, |direction, ruby| {
                    make_line_direction(direction.clone(), ruby)
                }),

                color_interpolation_method: CachedValue::new(
                    color_interpolation_method,
                    |method, ruby| {
                        YColorInterpolationMethod::new(*method).into_value_with(ruby)
                    },
                ),

                items: CachedValueList::new(items.to_vec(), |item, _ctx, ruby| {
                    make_gradient_length_percentage_item(item.clone(), ruby)
                }),

                repeating: flags.contains(GradientFlags::REPEATING),
                compat_mode,
            },
            _ => unreachable!("expected linear gradient"),
        }
    }

    pub fn repeating(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.repeating
    }

    pub fn compat_mode(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        compat_mode_to_id(rb_self.compat_mode, ruby)
    }

    pub fn direction(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.direction.get(ruby)
    }

    pub fn color_interpolation_method(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.color_interpolation_method.get(ruby)
    }

    pub fn items(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        rb_self.items.to_a(ruby)
    }
}

impl DataTypeFunctions for YLinearGradient {
    fn mark(&self, marker: &Marker) {
        self.direction.mark(marker);
        self.color_interpolation_method.mark(marker);
        self.items.mark(marker);
    }
}
