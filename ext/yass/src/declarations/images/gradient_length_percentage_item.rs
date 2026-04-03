use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc::Marker, typed_data};
use style::values::{generics::image::GradientItem, specified::{Color, LengthPercentage}};

use crate::{cached_value::CachedValue, declarations::{color::color::make_color, size::YLengthPercentage}};

pub fn make_gradient_length_percentage_item(item: GradientItem<Color, LengthPercentage>, ruby: &Ruby) -> Value {
    match item {
        GradientItem::SimpleColorStop(color) => {
            YSimpleColorStopLength::new(color).into_value_with(ruby)
        },
        GradientItem::ComplexColorStop { color, position } => {
            YComplexColorStopLength::new(color, position).into_value_with(ruby)
        },
        GradientItem::InterpolationHint(position) => {
            YInterpolationHintLength::new(position).into_value_with(ruby)
        },
    }
}

#[derive(magnus::TypedData)]
#[magnus(class = "Yass::Declarations::Image::Gradient::SimpleColorStopLength", mark)]
pub struct YSimpleColorStopLength {
    color: CachedValue<Color>,
}

impl YSimpleColorStopLength {
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
}

impl DataTypeFunctions for YSimpleColorStopLength {
    fn mark(&self, marker: &Marker) {
        self.color.mark(marker);
    }
}

#[derive(magnus::TypedData)]
#[magnus(class = "Yass::Declarations::Image::Gradient::ComplexColorStopLength", mark)]
pub struct YComplexColorStopLength {
    color: CachedValue<Color>,
    position: CachedValue<LengthPercentage>,
}

impl YComplexColorStopLength {
    pub fn new(color: Color, position: LengthPercentage) -> Self {
        Self {
            color: CachedValue::new(color, |color, ruby| {
                make_color(color, ruby)
            }),

            position: CachedValue::new(position, |position, ruby| {
                YLengthPercentage::new(position.clone()).into_value_with(ruby)
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

impl DataTypeFunctions for YComplexColorStopLength {
    fn mark(&self, marker: &Marker) {
        self.color.mark(marker);
        self.position.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Image::Gradient::InterpolationHintLength", mark)]
pub struct YInterpolationHintLength {
    position: CachedValue<LengthPercentage>,
}

impl YInterpolationHintLength {
    pub fn new(position: LengthPercentage) -> Self {
        Self {
            position: CachedValue::new(position, |position, ruby| {
                YLengthPercentage::new(position.clone()).into_value_with(ruby)
            })
        }
    }

    pub fn position(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.position.get(ruby)
    }
}

impl DataTypeFunctions for YInterpolationHintLength {
    fn mark(&self, marker: &magnus::gc::Marker) {
        self.position.mark(marker);
    }
}
