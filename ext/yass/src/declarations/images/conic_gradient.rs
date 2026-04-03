use magnus::{DataTypeFunctions, Error, IntoValue, RArray, Ruby, Value, gc::Marker, typed_data, value::Id};
use style::{color::mix::ColorInterpolationMethod, values::{generics::image::{GradientFlags, GradientItem}, specified::{Angle, AngleOrPercentage, Color, image::Gradient, position::Position}}};

use crate::{cached_value::CachedValue, cached_value_list::CachedValueList, declarations::{angle::YAngle, color::color_interpolation_method::YColorInterpolationMethod, images::{gradient_angle_or_percentage_item, position::YImagePosition}}};

#[derive(magnus::TypedData)]
#[magnus(class = "Yass::Declarations::Image::ConicGradient", mark)]
pub struct YConicGradient {
    angle: CachedValue<Angle>,
    position: CachedValue<Position>,
    color_interpolation_method: CachedValue<ColorInterpolationMethod>,
    items: CachedValueList<GradientItem<Color, AngleOrPercentage>>,
    repeating: bool,
}

impl YConicGradient {
    pub fn new(gradient: Gradient) -> Self {
        match gradient {
            Gradient::Conic {
                angle,
                position,
                color_interpolation_method,
                items,
                flags,
            } => Self {
                angle: CachedValue::new(angle, |angle, ruby| {
                    YAngle::new(*angle).into_value_with(ruby)
                }),

                position: CachedValue::new(position, |position, ruby| {
                    YImagePosition::new(position.clone()).into_value_with(ruby)
                }),

                color_interpolation_method: CachedValue::new(
                    color_interpolation_method,
                    |method, ruby| {
                        YColorInterpolationMethod::new(*method).into_value_with(ruby)
                    },
                ),

                items: CachedValueList::new(items.to_vec(), |item, _ctx, ruby| {
                    gradient_angle_or_percentage_item::make_gradient_angle_or_percentage_item(item.clone(), ruby)
                }),

                repeating: flags.contains(GradientFlags::REPEATING),
            },

            _ => unreachable!("expected conic gradient"),
        }
    }

    pub fn repeating(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.repeating
    }

    pub fn angle(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.angle.get(ruby)
    }

    pub fn position(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.position.get(ruby)
    }

    pub fn color_interpolation_method(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.color_interpolation_method.get(ruby)
    }

    pub fn items(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        rb_self.items.to_a(ruby)
    }
}

impl DataTypeFunctions for YConicGradient {
    fn mark(&self, marker: &Marker) {
        self.angle.mark(marker);
        self.position.mark(marker);
        self.color_interpolation_method.mark(marker);
        self.items.mark(marker);
    }
}
