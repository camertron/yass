use magnus::{DataTypeFunctions, Error, IntoValue, RArray, Ruby, Value, gc::Marker, typed_data, value::Id};
use style::{color::mix::ColorInterpolationMethod, values::{generics::image::{GradientCompatMode, GradientFlags, GradientItem}, specified::{Color, LengthPercentage, image::{EndingShape, Gradient}, position::Position}}};

use crate::{cached_value::CachedValue, cached_value_list::CachedValueList, declarations::{color::color_interpolation_method::YColorInterpolationMethod, images::{compat_mode_to_id, ending_shape::YEndingShape, gradient_length_percentage_item::make_gradient_length_percentage_item, position::YImagePosition}}};

#[derive(magnus::TypedData)]
#[magnus(class = "Yass::Declarations::Image::RadialGradient", mark)]
pub struct YImageRadialGradient {
    shape: CachedValue<EndingShape>,
    position: CachedValue<Position>,
    color_interpolation_method: CachedValue<ColorInterpolationMethod>,
    items: CachedValueList<GradientItem<Color, LengthPercentage>>,
    repeating: bool,
    compat_mode: GradientCompatMode,
}

impl YImageRadialGradient {
    pub fn new(gradient: Gradient) -> Self {
        match gradient {
            Gradient::Radial {
                shape,
                position,
                color_interpolation_method,
                items,
                flags,
                compat_mode,
            } => Self {
                shape: CachedValue::new(shape, |shape, ruby| {
                    YEndingShape::new(shape.clone()).into_value_with(ruby)
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
                    make_gradient_length_percentage_item(item, ruby)
                }),

                repeating: flags.contains(GradientFlags::REPEATING),
                compat_mode,
            },
            _ => unreachable!("expected radial gradient"),
        }
    }

    pub fn repeating(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.repeating
    }

    pub fn compat_mode(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        compat_mode_to_id(rb_self.compat_mode, ruby)
    }

    pub fn shape(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.shape.get(ruby)
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

impl DataTypeFunctions for YImageRadialGradient {
    fn mark(&self, marker: &Marker) {
        self.shape.mark(marker);
        self.position.mark(marker);
        self.color_interpolation_method.mark(marker);
        self.items.mark(marker);
    }
}
