use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc::Marker, typed_data, value::Id};
use style::values::{generics::{NonNegative, image::{Ellipse, ShapeExtent}}, specified::{LengthPercentage, NonNegativeLengthPercentage}};

use crate::{cached_value::CachedValue, declarations::{images::shape_extent_to_id, size::YLengthPercentage}};

pub fn make_ellipse(ellipse: &Ellipse<NonNegativeLengthPercentage>, ruby: &Ruby) -> Value {
    match ellipse {
        Ellipse::Radii(x, y) => {
            YRadiiEllipse::new(x.clone(), y.clone()).into_value_with(ruby)
        },

        Ellipse::Extent(shape) => {
            YExtentEllipse::new(shape.clone()).into_value_with(ruby)
        },
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Image::RadiiEllipse", mark)]
pub struct YRadiiEllipse {
    x: CachedValue<NonNegative<LengthPercentage>>,
    y: CachedValue<NonNegative<LengthPercentage>>
}

impl YRadiiEllipse {
    pub fn new(x: NonNegative<LengthPercentage>, y: NonNegative<LengthPercentage>) -> Self {
        Self {
            x: CachedValue::new(x, |x, ruby| {
                YLengthPercentage::new(x.0.clone()).into_value_with(ruby)
            }),

            y: CachedValue::new(y, |y, ruby| {
                YLengthPercentage::new(y.0.clone()).into_value_with(ruby)
            })
        }
    }

    pub fn x(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.x.get(ruby)
    }

    pub fn y(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.y.get(ruby)
    }
}

impl DataTypeFunctions for YRadiiEllipse {
    fn mark(&self, marker: &Marker) {
        self.x.mark(marker);
        self.y.mark(marker)
    }
}

#[magnus::wrap(class = "Yass::Declarations::Image::ExtentEllipse")]
pub struct YExtentEllipse {
    extent: ShapeExtent
}

impl YExtentEllipse {
    pub fn new(extent: ShapeExtent) -> Self {
        Self { extent }
    }

    pub fn extent(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        shape_extent_to_id(rb_self.extent, ruby)
    }
}
