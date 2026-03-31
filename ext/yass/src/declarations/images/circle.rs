use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc::Marker, typed_data, value::Id};
use style::values::{generics::{NonNegative, image::{Circle, ShapeExtent}}, specified::{Length, NonNegativeLength}};

use crate::{cached_value::CachedValue, declarations::{calc::YCalc, length::no_calc_length_to_value}};

pub fn make_circle(circle: &Circle<NonNegativeLength>, ruby: &Ruby) -> Value {
    match circle {
        Circle::Radius(length) => {
            YRadiusCircle::new(length.clone()).into_value_with(ruby)
        },

        Circle::Extent(shape_extent) => {
            YExtentCircle::new(shape_extent.clone()).into_value_with(ruby)
        },
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Image::RadiusCircle", mark)]
pub struct YRadiusCircle {
    length: CachedValue<NonNegative<Length>>
}

impl YRadiusCircle {
    pub fn new(length: NonNegative<Length>) -> Self {
        Self {
            length: CachedValue::new(length, |length, ruby| {
                match &length.0 {
                    Length::NoCalc(no_calc_length) => {
                        no_calc_length_to_value(no_calc_length, ruby)
                    },

                    Length::Calc(calc_length_percentage) => {
                        YCalc::new(calc_length_percentage.clone()).into_value_with(ruby)
                    },
                }
            })
        }
    }

    pub fn length(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.length.get(ruby)
    }
}

impl DataTypeFunctions for YRadiusCircle {
    fn mark(&self, marker: &Marker) {
        self.length.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::Declarations::Image::ExtentCircle")]
pub struct YExtentCircle {
    extent: ShapeExtent
}

impl YExtentCircle {
    pub fn new(extent: ShapeExtent) -> Self {
        Self { extent }
    }

    pub fn extent(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.extent {
            ShapeExtent::ClosestSide => ruby.intern("closest_side"),
            ShapeExtent::FarthestSide => ruby.intern("farthest_side"),
            ShapeExtent::ClosestCorner => ruby.intern("closest_corner"),
            ShapeExtent::FarthestCorner => ruby.intern("farthest_corner"),
            ShapeExtent::Contain => ruby.intern("contain"),
            ShapeExtent::Cover => ruby.intern("cover"),
        }
    }
}
