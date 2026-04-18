use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc::Marker, typed_data};
use style::values::{generics::image::GradientItem, specified::{AngleOrPercentage, Color}};

use crate::{cached_value::CachedValue, declarations::{color::color::make_color, images::angle_or_percentage::YAngleOrPercentage}};

pub fn make_gradient_angle_or_percentage_item(item: &GradientItem<Color, AngleOrPercentage>, ruby: &Ruby) -> Value {
    match item {
        GradientItem::SimpleColorStop(color) => {
            YSimpleColorStopAngle::new(color.clone()).into_value_with(ruby)
        },
        GradientItem::ComplexColorStop { color, position } => {
            YComplexColorStopAngle::new(color.clone(), position.clone()).into_value_with(ruby)
        },
        GradientItem::InterpolationHint(position) => {
            YInterpolationHintAngle::new(position.clone()).into_value_with(ruby)
        },
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Image::Gradient::SimpleColorStopAngle", mark)]
pub struct YSimpleColorStopAngle {
    color: CachedValue<Color>,
}

impl YSimpleColorStopAngle {
    pub fn new(color: Color) -> Self {
        Self {
            color: CachedValue::new(color, |color, ruby| {
                make_color(color, ruby)
            }),
        }
    }

    pub fn color(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.color.get(ruby)
    }

    pub fn position(ruby: &Ruby, _rb_self: typed_data::Obj<Self>) -> Value {
        ruby.qnil().into_value_with(ruby)
    }
}

impl DataTypeFunctions for YSimpleColorStopAngle {
    fn mark(&self, marker: &Marker) {
        self.color.mark(marker);
    }
}

#[derive(magnus::TypedData)]
#[magnus(class = "Yass::Declarations::Image::Gradient::ComplexColorStopAngle", mark)]
pub struct YComplexColorStopAngle {
    color: CachedValue<Color>,
    position: CachedValue<AngleOrPercentage>,
}

impl YComplexColorStopAngle {
    pub fn new(color: Color, position: AngleOrPercentage) -> Self {
        Self {
            color: CachedValue::new(color, |color, ruby| {
                make_color(color, ruby)
            }),

            position: CachedValue::new(position, |position, ruby| {
                YAngleOrPercentage::new(*position).into_value_with(ruby)
            }),
        }
    }

    pub fn color(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.color.get(ruby)
    }

    pub fn position(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.position.get(ruby)
    }
}

impl DataTypeFunctions for YComplexColorStopAngle {
    fn mark(&self, marker: &Marker) {
        self.color.mark(marker);
        self.position.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::Declarations::Image::Gradient::InterpolationHintAngle")]
pub struct YInterpolationHintAngle {
    position: AngleOrPercentage,
}

impl YInterpolationHintAngle {
    pub fn new(position: AngleOrPercentage) -> Self {
        Self { position }
    }

    pub fn color(ruby: &Ruby, _rb_self: typed_data::Obj<Self>) -> Value {
        ruby.qnil().into_value_with(ruby)
    }

    pub fn position(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        YAngleOrPercentage::new(rb_self.position).into_value_with(ruby)
    }
}
