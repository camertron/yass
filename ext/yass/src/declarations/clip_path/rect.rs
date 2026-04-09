use style::values::generics::basic_shape::InsetRect;

use super::*;

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::ClipPath::Rect", mark)]
pub struct YClipPathRect {
    rect: CachedValue<BasicShapeRect>,
}

impl YClipPathRect {
    pub fn new(rect: BasicShapeRect) -> Self {
        Self {
            rect: CachedValue::new(rect, make_basic_shape_rect),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.rect.get(ruby)
    }
}

impl DataTypeFunctions for YClipPathRect {
    fn mark(&self, marker: &gc::Marker) {
        self.rect.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::ClipPath::InsetRect", mark)]
pub struct YClipPathInsetRect {
    top: CachedValue<LengthPercentage>,
    right: CachedValue<LengthPercentage>,
    bottom: CachedValue<LengthPercentage>,
    left: CachedValue<LengthPercentage>,
    round: CachedValue<ClipPathBorderRadius>,
}

impl YClipPathInsetRect {
    pub fn new(inset_rect: InsetRect<LengthPercentage>) -> Self {
        Self {
            top: CachedValue::new(inset_rect.rect.0, |value, ruby| {
                length_percentage_to_value(value, ruby)
            }),

            right: CachedValue::new(inset_rect.rect.1, |value, ruby| {
                length_percentage_to_value(value, ruby)
            }),

            bottom: CachedValue::new(inset_rect.rect.2, |value, ruby| {
                length_percentage_to_value(value, ruby)
            }),

            left: CachedValue::new(inset_rect.rect.3, |value, ruby| {
                length_percentage_to_value(value, ruby)
            }),

            round: CachedValue::new(inset_rect.round, make_border_radius),
        }
    }

    pub fn top(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.top.get(ruby)
    }

    pub fn right(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.right.get(ruby)
    }

    pub fn bottom(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.bottom.get(ruby)
    }

    pub fn left(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.left.get(ruby)
    }

    pub fn round(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.round.get(ruby)
    }
}

impl DataTypeFunctions for YClipPathInsetRect {
    fn mark(&self, marker: &gc::Marker) {
        self.top.mark(marker);
        self.right.mark(marker);
        self.bottom.mark(marker);
        self.left.mark(marker);
        self.round.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::ClipPath::XywhRect", mark)]
pub struct YClipPathXywhRect {
    x: CachedValue<LengthPercentage>,
    y: CachedValue<LengthPercentage>,
    width: CachedValue<NonNegative<LengthPercentage>>,
    height: CachedValue<NonNegative<LengthPercentage>>,
    round: CachedValue<ClipPathBorderRadius>,
}

impl YClipPathXywhRect {
    pub fn new(xywh: Xywh) -> Self {
        Self {
            x: CachedValue::new(xywh.x, |value, ruby| {
                length_percentage_to_value(value, ruby)
            }),

            y: CachedValue::new(xywh.y, |value, ruby| {
                length_percentage_to_value(value, ruby)
            }),

            width: CachedValue::new(xywh.width, |value, ruby| {
                length_percentage_to_value(&value.0, ruby)
            }),

            height: CachedValue::new(xywh.height, |value, ruby| {
                length_percentage_to_value(&value.0, ruby)
            }),

            round: CachedValue::new(xywh.round, make_border_radius),
        }
    }

    pub fn x(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.x.get(ruby)
    }

    pub fn y(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.y.get(ruby)
    }

    pub fn width(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.width.get(ruby)
    }

    pub fn height(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.height.get(ruby)
    }

    pub fn round(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.round.get(ruby)
    }
}

impl DataTypeFunctions for YClipPathXywhRect {
    fn mark(&self, marker: &gc::Marker) {
        self.x.mark(marker);
        self.y.mark(marker);
        self.width.mark(marker);
        self.height.mark(marker);
        self.round.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::ClipPath::RectFunction", mark)]
pub struct YClipPathRectFunction {
    top: CachedValue<LengthPercentageOrAuto>,
    right: CachedValue<LengthPercentageOrAuto>,
    bottom: CachedValue<LengthPercentageOrAuto>,
    left: CachedValue<LengthPercentageOrAuto>,
    round: CachedValue<ClipPathBorderRadius>,
}

impl YClipPathRectFunction {
    pub fn new(shape_rect: ShapeRectFunction) -> Self {
        Self {
            top: CachedValue::new(shape_rect.rect.0, make_length_percentage_or_auto),
            right: CachedValue::new(shape_rect.rect.1, make_length_percentage_or_auto),
            bottom: CachedValue::new(shape_rect.rect.2, make_length_percentage_or_auto),
            left: CachedValue::new(shape_rect.rect.3, make_length_percentage_or_auto),
            round: CachedValue::new(shape_rect.round, make_border_radius),
        }
    }

    pub fn top(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.top.get(ruby)
    }

    pub fn right(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.right.get(ruby)
    }

    pub fn bottom(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.bottom.get(ruby)
    }

    pub fn left(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.left.get(ruby)
    }

    pub fn round(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.round.get(ruby)
    }
}

impl DataTypeFunctions for YClipPathRectFunction {
    fn mark(&self, marker: &gc::Marker) {
        self.top.mark(marker);
        self.right.mark(marker);
        self.bottom.mark(marker);
        self.left.mark(marker);
        self.round.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::ClipPath::BorderRadius", mark)]
pub struct YClipPathBorderRadius {
    top_left: CachedValue<ClipPathBorderCornerRadius>,
    top_right: CachedValue<ClipPathBorderCornerRadius>,
    bottom_right: CachedValue<ClipPathBorderCornerRadius>,
    bottom_left: CachedValue<ClipPathBorderCornerRadius>,
}

impl YClipPathBorderRadius {
    pub fn new(radius: ClipPathBorderRadius) -> Self {
        Self {
            top_left: CachedValue::new(radius.top_left, make_border_corner_radius),
            top_right: CachedValue::new(radius.top_right, make_border_corner_radius),
            bottom_right: CachedValue::new(radius.bottom_right, make_border_corner_radius),
            bottom_left: CachedValue::new(radius.bottom_left, make_border_corner_radius),
        }
    }

    pub fn top_left(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.top_left.get(ruby)
    }

    pub fn top_right(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.top_right.get(ruby)
    }

    pub fn bottom_right(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.bottom_right.get(ruby)
    }

    pub fn bottom_left(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.bottom_left.get(ruby)
    }
}

impl DataTypeFunctions for YClipPathBorderRadius {
    fn mark(&self, marker: &gc::Marker) {
        self.top_left.mark(marker);
        self.top_right.mark(marker);
        self.bottom_right.mark(marker);
        self.bottom_left.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::ClipPath::BorderCornerRadius", mark)]
pub struct YClipPathBorderCornerRadius {
    width: CachedValue<NonNegative<LengthPercentage>>,
    height: CachedValue<NonNegative<LengthPercentage>>,
}

impl YClipPathBorderCornerRadius {
    pub fn new(corner: ClipPathBorderCornerRadius) -> Self {
        Self {
            width: CachedValue::new(corner.0.width, |width, ruby| {
                YLengthPercentage::new(width.0.clone()).into_value_with(ruby)
            }),

            height: CachedValue::new(corner.0.height, |height, ruby| {
                YLengthPercentage::new(height.0.clone()).into_value_with(ruby)
            }),
        }
    }

    pub fn width(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.width.get(ruby)
    }

    pub fn height(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.height.get(ruby)
    }
}

impl DataTypeFunctions for YClipPathBorderCornerRadius {
    fn mark(&self, marker: &gc::Marker) {
        self.width.mark(marker);
        self.height.mark(marker);
    }
}
