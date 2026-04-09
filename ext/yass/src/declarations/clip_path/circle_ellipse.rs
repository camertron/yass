use super::*;

#[magnus::wrap(class = "Yass::Declarations::ClipPath::Auto")]
pub struct YClipPathAuto {}

impl YClipPathAuto {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::ClipPath::Circle", mark)]
pub struct YClipPathCircle {
    position: CachedValue<ClipPathPositionOrAuto>,
    radius: CachedValue<ClipPathShapeRadius>,
}

impl YClipPathCircle {
    pub fn new(circle: style::values::generics::basic_shape::Circle<LengthPercentage>) -> Self {
        Self {
            position: CachedValue::new(circle.position, make_shape_position_or_auto),
            radius: CachedValue::new(circle.radius, make_shape_radius),
        }
    }

    pub fn position(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.position.get(ruby)
    }

    pub fn radius(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.radius.get(ruby)
    }
}

impl DataTypeFunctions for YClipPathCircle {
    fn mark(&self, marker: &gc::Marker) {
        self.position.mark(marker);
        self.radius.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::ClipPath::Ellipse", mark)]
pub struct YClipPathEllipse {
    position: CachedValue<ClipPathPositionOrAuto>,
    x_radius: CachedValue<ClipPathShapeRadius>,
    y_radius: CachedValue<ClipPathShapeRadius>,
}

impl YClipPathEllipse {
    pub fn new(ellipse: style::values::generics::basic_shape::Ellipse<LengthPercentage>) -> Self {
        Self {
            position: CachedValue::new(ellipse.position, make_shape_position_or_auto),
            x_radius: CachedValue::new(ellipse.semiaxis_x, make_shape_radius),
            y_radius: CachedValue::new(ellipse.semiaxis_y, make_shape_radius),
        }
    }

    pub fn position(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.position.get(ruby)
    }

    pub fn x_radius(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.x_radius.get(ruby)
    }

    pub fn y_radius(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.y_radius.get(ruby)
    }
}

impl DataTypeFunctions for YClipPathEllipse {
    fn mark(&self, marker: &gc::Marker) {
        self.position.mark(marker);
        self.x_radius.mark(marker);
        self.y_radius.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::ClipPath::Position", mark)]
pub struct YClipPathPosition {
    x: CachedValue<LengthPercentage>,
    y: CachedValue<LengthPercentage>,
}

impl YClipPathPosition {
    pub fn new(position: ClipPathShapePosition) -> Self {
        Self {
            x: CachedValue::new(position.horizontal, |value, ruby| {
                length_percentage_to_value(value, ruby)
            }),

            y: CachedValue::new(position.vertical, |value, ruby| {
                length_percentage_to_value(value, ruby)
            }),
        }
    }

    pub fn x(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.x.get(ruby)
    }

    pub fn y(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.y.get(ruby)
    }
}

impl DataTypeFunctions for YClipPathPosition {
    fn mark(&self, marker: &gc::Marker) {
        self.x.mark(marker);
        self.y.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::Declarations::ClipPath::PositionAuto")]
pub struct YClipPathPositionAuto {}

impl YClipPathPositionAuto {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::ClipPath::ShapeRadiusLength", mark)]
pub struct YClipPathShapeRadiusLength {
    value: CachedValue<NonNegative<LengthPercentage>>,
}

impl YClipPathShapeRadiusLength {
    pub fn new(value: NonNegative<LengthPercentage>) -> Self {
        Self {
            value: CachedValue::new(value, |value, ruby| {
                length_percentage_to_value(&value.0, ruby)
            }),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.value.get(ruby)
    }
}

impl DataTypeFunctions for YClipPathShapeRadiusLength {
    fn mark(&self, marker: &gc::Marker) {
        self.value.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::Declarations::ClipPath::ShapeRadiusClosestSide")]
pub struct YClipPathShapeRadiusClosestSide {}

impl YClipPathShapeRadiusClosestSide {
    pub fn new() -> Self {
        Self {}
    }
}

#[magnus::wrap(class = "Yass::Declarations::ClipPath::ShapeRadiusFarthestSide")]
pub struct YClipPathShapeRadiusFarthestSide {}

impl YClipPathShapeRadiusFarthestSide {
    pub fn new() -> Self {
        Self {}
    }
}
