use magnus::{DataTypeFunctions, IntoValue, Ruby, Value, gc::Marker, typed_data};
use style::values::specified::image::EndingShape;

use crate::{cached_value::CachedValue, declarations::images::{circle::make_circle, ellipse::make_ellipse}};

#[derive(magnus::TypedData)]
#[magnus(class = "Yass::Declarations::Image::EndingShape", mark)]
pub struct YEndingShape {
    shape: CachedValue<EndingShape>
}

impl YEndingShape {
    pub fn new(shape: EndingShape) -> Self {
        Self {
            shape: CachedValue::new(shape, |shape, ruby| {
                match shape {
                    EndingShape::Circle(circle) => {
                        make_circle(circle, ruby).into_value_with(ruby)
                    },

                    EndingShape::Ellipse(ellipse) => {
                        make_ellipse(ellipse.clone(), ruby).into_value_with(ruby)
                    },
                }
            })
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.shape.get(ruby)
    }
}

impl DataTypeFunctions for YEndingShape {
    fn mark(&self, marker: &Marker) {
        self.shape.mark(marker);
    }
}
