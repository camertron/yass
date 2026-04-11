use magnus::{DataTypeFunctions, Error, IntoValue, RArray, Ruby, TypedData, Value, gc, typed_data};
use style::values::{
    computed::Percentage,
    generics::transform::{Matrix, Matrix3D, PerspectiveFunction},
    specified::{Angle, Integer, Length, LengthPercentage, Number, Transform, transform::TransformOperation},
};

use crate::{
    cached_value::CachedValue,
    cached_value_list::CachedValueList,
    declarations::{
        angle::YAngle,
        length::length_to_value,
        number::YNumber,
        percentage::YPercentage,
        size::YLengthPercentage,
    },
};

fn number_to_value(number: &Number, ruby: &Ruby) -> Value {
    YNumber::new(number.get()).into_value_with(ruby)
}

fn angle_to_value(angle: &Angle, ruby: &Ruby) -> Value {
    YAngle::new(*angle).into_value_with(ruby)
}

fn length_percentage_to_value(length_percentage: &LengthPercentage, ruby: &Ruby) -> Value {
    YLengthPercentage::new(length_percentage.clone()).into_value_with(ruby)
}

fn make_perspective_value(perspective_function: &PerspectiveFunction<Length>, ruby: &Ruby) -> Value {
    match perspective_function {
        PerspectiveFunction::None => YTransformPerspectiveNone::new().into_value_with(ruby),
        PerspectiveFunction::Length(length) => {
            YTransformPerspectiveLength::new(length.clone()).into_value_with(ruby)
        }
    }
}

fn make_transform_operation(operation: &TransformOperation, ruby: &Ruby) -> Value {
    match operation {
        TransformOperation::Matrix(matrix) => YTransformMatrix::new(*matrix).into_value_with(ruby),
        TransformOperation::Matrix3D(matrix) => {
            YTransformMatrix3D::new(*matrix).into_value_with(ruby)
        }
        TransformOperation::Skew(x, y) => YTransformSkew::new(*x, *y).into_value_with(ruby),
        TransformOperation::SkewX(angle) => YTransformSkewX::new(*angle).into_value_with(ruby),
        TransformOperation::SkewY(angle) => YTransformSkewY::new(*angle).into_value_with(ruby),
        TransformOperation::Translate(x, y) => {
            YTransformTranslate::new(x.clone(), y.clone()).into_value_with(ruby)
        }
        TransformOperation::TranslateX(value) => {
            YTransformTranslateX::new(value.clone()).into_value_with(ruby)
        }
        TransformOperation::TranslateY(value) => {
            YTransformTranslateY::new(value.clone()).into_value_with(ruby)
        }
        TransformOperation::TranslateZ(value) => {
            YTransformTranslateZ::new(value.clone()).into_value_with(ruby)
        }
        TransformOperation::Translate3D(x, y, z) => {
            YTransformTranslate3D::new(x.clone(), y.clone(), z.clone()).into_value_with(ruby)
        }
        TransformOperation::Scale(x, y) => YTransformScale::new(*x, *y).into_value_with(ruby),
        TransformOperation::ScaleX(value) => YTransformScaleX::new(*value).into_value_with(ruby),
        TransformOperation::ScaleY(value) => YTransformScaleY::new(*value).into_value_with(ruby),
        TransformOperation::ScaleZ(value) => YTransformScaleZ::new(*value).into_value_with(ruby),
        TransformOperation::Scale3D(x, y, z) => {
            YTransformScale3D::new(*x, *y, *z).into_value_with(ruby)
        }
        TransformOperation::Rotate(angle) => YTransformRotate::new(*angle).into_value_with(ruby),
        TransformOperation::RotateX(angle) => YTransformRotateX::new(*angle).into_value_with(ruby),
        TransformOperation::RotateY(angle) => YTransformRotateY::new(*angle).into_value_with(ruby),
        TransformOperation::RotateZ(angle) => YTransformRotateZ::new(*angle).into_value_with(ruby),
        TransformOperation::Rotate3D(x, y, z, angle) => {
            YTransformRotate3D::new(*x, *y, *z, *angle).into_value_with(ruby)
        }
        TransformOperation::Perspective(perspective_function) => {
            YTransformPerspective::new(perspective_function.clone()).into_value_with(ruby)
        }
        TransformOperation::InterpolateMatrix {
            from_list,
            to_list,
            progress,
        } => YTransformInterpolateMatrix::new(
            from_list.clone(),
            to_list.clone(),
            *progress,
        )
        .into_value_with(ruby),
        TransformOperation::AccumulateMatrix {
            from_list,
            to_list,
            count,
        } => YTransformAccumulateMatrix::new(from_list.clone(), to_list.clone(), *count)
            .into_value_with(ruby),
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Transform", mark)]
pub struct YTransform {
    operations: CachedValueList<TransformOperation>,
}

impl YTransform {
    pub fn new(transform: Transform) -> Self {
        Self {
            operations: CachedValueList::new(transform.0.to_vec(), |operation, _ctx, ruby| {
                make_transform_operation(operation, ruby)
            }),
        }
    }

    pub fn operations(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        rb_self.operations.to_a(ruby)
    }
}

impl DataTypeFunctions for YTransform {
    fn mark(&self, marker: &gc::Marker) {
        self.operations.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Transform::Matrix", mark)]
pub struct YTransformMatrix {
    a: CachedValue<Number>,
    b: CachedValue<Number>,
    c: CachedValue<Number>,
    d: CachedValue<Number>,
    e: CachedValue<Number>,
    f: CachedValue<Number>,
}

impl YTransformMatrix {
    pub fn new(matrix: Matrix<Number>) -> Self {
        Self {
            a: CachedValue::new(matrix.a, number_to_value),
            b: CachedValue::new(matrix.b, number_to_value),
            c: CachedValue::new(matrix.c, number_to_value),
            d: CachedValue::new(matrix.d, number_to_value),
            e: CachedValue::new(matrix.e, number_to_value),
            f: CachedValue::new(matrix.f, number_to_value),
        }
    }

    pub fn a(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.a.get(ruby)
    }

    pub fn b(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.b.get(ruby)
    }

    pub fn c(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.c.get(ruby)
    }

    pub fn d(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.d.get(ruby)
    }

    pub fn e(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.e.get(ruby)
    }

    pub fn f(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.f.get(ruby)
    }
}

impl DataTypeFunctions for YTransformMatrix {
    fn mark(&self, marker: &gc::Marker) {
        self.a.mark(marker);
        self.b.mark(marker);
        self.c.mark(marker);
        self.d.mark(marker);
        self.e.mark(marker);
        self.f.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Transform::Matrix3D", mark)]
pub struct YTransformMatrix3D {
    m11: CachedValue<Number>,
    m12: CachedValue<Number>,
    m13: CachedValue<Number>,
    m14: CachedValue<Number>,
    m21: CachedValue<Number>,
    m22: CachedValue<Number>,
    m23: CachedValue<Number>,
    m24: CachedValue<Number>,
    m31: CachedValue<Number>,
    m32: CachedValue<Number>,
    m33: CachedValue<Number>,
    m34: CachedValue<Number>,
    m41: CachedValue<Number>,
    m42: CachedValue<Number>,
    m43: CachedValue<Number>,
    m44: CachedValue<Number>,
}

impl YTransformMatrix3D {
    pub fn new(matrix: Matrix3D<Number>) -> Self {
        Self {
            m11: CachedValue::new(matrix.m11, number_to_value),
            m12: CachedValue::new(matrix.m12, number_to_value),
            m13: CachedValue::new(matrix.m13, number_to_value),
            m14: CachedValue::new(matrix.m14, number_to_value),
            m21: CachedValue::new(matrix.m21, number_to_value),
            m22: CachedValue::new(matrix.m22, number_to_value),
            m23: CachedValue::new(matrix.m23, number_to_value),
            m24: CachedValue::new(matrix.m24, number_to_value),
            m31: CachedValue::new(matrix.m31, number_to_value),
            m32: CachedValue::new(matrix.m32, number_to_value),
            m33: CachedValue::new(matrix.m33, number_to_value),
            m34: CachedValue::new(matrix.m34, number_to_value),
            m41: CachedValue::new(matrix.m41, number_to_value),
            m42: CachedValue::new(matrix.m42, number_to_value),
            m43: CachedValue::new(matrix.m43, number_to_value),
            m44: CachedValue::new(matrix.m44, number_to_value),
        }
    }

    pub fn m11(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value { rb_self.m11.get(ruby) }
    pub fn m12(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value { rb_self.m12.get(ruby) }
    pub fn m13(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value { rb_self.m13.get(ruby) }
    pub fn m14(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value { rb_self.m14.get(ruby) }
    pub fn m21(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value { rb_self.m21.get(ruby) }
    pub fn m22(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value { rb_self.m22.get(ruby) }
    pub fn m23(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value { rb_self.m23.get(ruby) }
    pub fn m24(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value { rb_self.m24.get(ruby) }
    pub fn m31(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value { rb_self.m31.get(ruby) }
    pub fn m32(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value { rb_self.m32.get(ruby) }
    pub fn m33(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value { rb_self.m33.get(ruby) }
    pub fn m34(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value { rb_self.m34.get(ruby) }
    pub fn m41(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value { rb_self.m41.get(ruby) }
    pub fn m42(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value { rb_self.m42.get(ruby) }
    pub fn m43(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value { rb_self.m43.get(ruby) }
    pub fn m44(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value { rb_self.m44.get(ruby) }
}

impl DataTypeFunctions for YTransformMatrix3D {
    fn mark(&self, marker: &gc::Marker) {
        self.m11.mark(marker);
        self.m12.mark(marker);
        self.m13.mark(marker);
        self.m14.mark(marker);
        self.m21.mark(marker);
        self.m22.mark(marker);
        self.m23.mark(marker);
        self.m24.mark(marker);
        self.m31.mark(marker);
        self.m32.mark(marker);
        self.m33.mark(marker);
        self.m34.mark(marker);
        self.m41.mark(marker);
        self.m42.mark(marker);
        self.m43.mark(marker);
        self.m44.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Transform::Skew", mark)]
pub struct YTransformSkew {
    x: CachedValue<Angle>,
    y: CachedValue<Angle>,
}

impl YTransformSkew {
    pub fn new(x: Angle, y: Angle) -> Self {
        Self {
            x: CachedValue::new(x, angle_to_value),
            y: CachedValue::new(y, angle_to_value),
        }
    }

    pub fn x(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value { rb_self.x.get(ruby) }
    pub fn y(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value { rb_self.y.get(ruby) }
}

impl DataTypeFunctions for YTransformSkew {
    fn mark(&self, marker: &gc::Marker) {
        self.x.mark(marker);
        self.y.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Transform::SkewX", mark)]
pub struct YTransformSkewX {
    angle: CachedValue<Angle>,
}

impl YTransformSkewX {
    pub fn new(angle: Angle) -> Self {
        Self {
            angle: CachedValue::new(angle, angle_to_value),
        }
    }

    pub fn angle(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value { rb_self.angle.get(ruby) }
}

impl DataTypeFunctions for YTransformSkewX {
    fn mark(&self, marker: &gc::Marker) {
        self.angle.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Transform::SkewY", mark)]
pub struct YTransformSkewY {
    angle: CachedValue<Angle>,
}

impl YTransformSkewY {
    pub fn new(angle: Angle) -> Self {
        Self {
            angle: CachedValue::new(angle, angle_to_value),
        }
    }

    pub fn angle(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value { rb_self.angle.get(ruby) }
}

impl DataTypeFunctions for YTransformSkewY {
    fn mark(&self, marker: &gc::Marker) {
        self.angle.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Transform::Translate", mark)]
pub struct YTransformTranslate {
    x: CachedValue<LengthPercentage>,
    y: CachedValue<LengthPercentage>,
}

impl YTransformTranslate {
    pub fn new(x: LengthPercentage, y: LengthPercentage) -> Self {
        Self {
            x: CachedValue::new(x, length_percentage_to_value),
            y: CachedValue::new(y, length_percentage_to_value),
        }
    }

    pub fn x(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value { rb_self.x.get(ruby) }
    pub fn y(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value { rb_self.y.get(ruby) }
}

impl DataTypeFunctions for YTransformTranslate {
    fn mark(&self, marker: &gc::Marker) {
        self.x.mark(marker);
        self.y.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Transform::TranslateX", mark)]
pub struct YTransformTranslateX {
    value: CachedValue<LengthPercentage>,
}

impl YTransformTranslateX {
    pub fn new(value: LengthPercentage) -> Self {
        Self {
            value: CachedValue::new(value, length_percentage_to_value),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value { rb_self.value.get(ruby) }
}

impl DataTypeFunctions for YTransformTranslateX {
    fn mark(&self, marker: &gc::Marker) {
        self.value.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Transform::TranslateY", mark)]
pub struct YTransformTranslateY {
    value: CachedValue<LengthPercentage>,
}

impl YTransformTranslateY {
    pub fn new(value: LengthPercentage) -> Self {
        Self {
            value: CachedValue::new(value, length_percentage_to_value),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value { rb_self.value.get(ruby) }
}

impl DataTypeFunctions for YTransformTranslateY {
    fn mark(&self, marker: &gc::Marker) {
        self.value.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Transform::TranslateZ", mark)]
pub struct YTransformTranslateZ {
    value: CachedValue<Length>,
}

impl YTransformTranslateZ {
    pub fn new(value: Length) -> Self {
        Self {
            value: CachedValue::new(value, |length, ruby| length_to_value(length, ruby)),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value { rb_self.value.get(ruby) }
}

impl DataTypeFunctions for YTransformTranslateZ {
    fn mark(&self, marker: &gc::Marker) {
        self.value.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Transform::Translate3D", mark)]
pub struct YTransformTranslate3D {
    x: CachedValue<LengthPercentage>,
    y: CachedValue<LengthPercentage>,
    z: CachedValue<Length>,
}

impl YTransformTranslate3D {
    pub fn new(x: LengthPercentage, y: LengthPercentage, z: Length) -> Self {
        Self {
            x: CachedValue::new(x, length_percentage_to_value),
            y: CachedValue::new(y, length_percentage_to_value),
            z: CachedValue::new(z, |length, ruby| length_to_value(length, ruby)),
        }
    }

    pub fn x(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value { rb_self.x.get(ruby) }
    pub fn y(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value { rb_self.y.get(ruby) }
    pub fn z(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value { rb_self.z.get(ruby) }
}

impl DataTypeFunctions for YTransformTranslate3D {
    fn mark(&self, marker: &gc::Marker) {
        self.x.mark(marker);
        self.y.mark(marker);
        self.z.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Transform::Scale", mark)]
pub struct YTransformScale {
    x: CachedValue<Number>,
    y: CachedValue<Number>,
}

impl YTransformScale {
    pub fn new(x: Number, y: Number) -> Self {
        Self {
            x: CachedValue::new(x, number_to_value),
            y: CachedValue::new(y, number_to_value),
        }
    }

    pub fn x(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value { rb_self.x.get(ruby) }
    pub fn y(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value { rb_self.y.get(ruby) }
}

impl DataTypeFunctions for YTransformScale {
    fn mark(&self, marker: &gc::Marker) {
        self.x.mark(marker);
        self.y.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Transform::ScaleX", mark)]
pub struct YTransformScaleX {
    value: CachedValue<Number>,
}

impl YTransformScaleX {
    pub fn new(value: Number) -> Self {
        Self {
            value: CachedValue::new(value, number_to_value),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value { rb_self.value.get(ruby) }
}

impl DataTypeFunctions for YTransformScaleX {
    fn mark(&self, marker: &gc::Marker) {
        self.value.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Transform::ScaleY", mark)]
pub struct YTransformScaleY {
    value: CachedValue<Number>,
}

impl YTransformScaleY {
    pub fn new(value: Number) -> Self {
        Self {
            value: CachedValue::new(value, number_to_value),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value { rb_self.value.get(ruby) }
}

impl DataTypeFunctions for YTransformScaleY {
    fn mark(&self, marker: &gc::Marker) {
        self.value.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Transform::ScaleZ", mark)]
pub struct YTransformScaleZ {
    value: CachedValue<Number>,
}

impl YTransformScaleZ {
    pub fn new(value: Number) -> Self {
        Self {
            value: CachedValue::new(value, number_to_value),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value { rb_self.value.get(ruby) }
}

impl DataTypeFunctions for YTransformScaleZ {
    fn mark(&self, marker: &gc::Marker) {
        self.value.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Transform::Scale3D", mark)]
pub struct YTransformScale3D {
    x: CachedValue<Number>,
    y: CachedValue<Number>,
    z: CachedValue<Number>,
}

impl YTransformScale3D {
    pub fn new(x: Number, y: Number, z: Number) -> Self {
        Self {
            x: CachedValue::new(x, number_to_value),
            y: CachedValue::new(y, number_to_value),
            z: CachedValue::new(z, number_to_value),
        }
    }

    pub fn x(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value { rb_self.x.get(ruby) }
    pub fn y(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value { rb_self.y.get(ruby) }
    pub fn z(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value { rb_self.z.get(ruby) }
}

impl DataTypeFunctions for YTransformScale3D {
    fn mark(&self, marker: &gc::Marker) {
        self.x.mark(marker);
        self.y.mark(marker);
        self.z.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Transform::Rotate", mark)]
pub struct YTransformRotate {
    angle: CachedValue<Angle>,
}

impl YTransformRotate {
    pub fn new(angle: Angle) -> Self {
        Self {
            angle: CachedValue::new(angle, angle_to_value),
        }
    }

    pub fn angle(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value { rb_self.angle.get(ruby) }
}

impl DataTypeFunctions for YTransformRotate {
    fn mark(&self, marker: &gc::Marker) {
        self.angle.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Transform::RotateX", mark)]
pub struct YTransformRotateX {
    angle: CachedValue<Angle>,
}

impl YTransformRotateX {
    pub fn new(angle: Angle) -> Self {
        Self {
            angle: CachedValue::new(angle, angle_to_value),
        }
    }

    pub fn angle(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value { rb_self.angle.get(ruby) }
}

impl DataTypeFunctions for YTransformRotateX {
    fn mark(&self, marker: &gc::Marker) {
        self.angle.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Transform::RotateY", mark)]
pub struct YTransformRotateY {
    angle: CachedValue<Angle>,
}

impl YTransformRotateY {
    pub fn new(angle: Angle) -> Self {
        Self {
            angle: CachedValue::new(angle, angle_to_value),
        }
    }

    pub fn angle(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value { rb_self.angle.get(ruby) }
}

impl DataTypeFunctions for YTransformRotateY {
    fn mark(&self, marker: &gc::Marker) {
        self.angle.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Transform::RotateZ", mark)]
pub struct YTransformRotateZ {
    angle: CachedValue<Angle>,
}

impl YTransformRotateZ {
    pub fn new(angle: Angle) -> Self {
        Self {
            angle: CachedValue::new(angle, angle_to_value),
        }
    }

    pub fn angle(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value { rb_self.angle.get(ruby) }
}

impl DataTypeFunctions for YTransformRotateZ {
    fn mark(&self, marker: &gc::Marker) {
        self.angle.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Transform::Rotate3D", mark)]
pub struct YTransformRotate3D {
    x: CachedValue<Number>,
    y: CachedValue<Number>,
    z: CachedValue<Number>,
    angle: CachedValue<Angle>,
}

impl YTransformRotate3D {
    pub fn new(x: Number, y: Number, z: Number, angle: Angle) -> Self {
        Self {
            x: CachedValue::new(x, number_to_value),
            y: CachedValue::new(y, number_to_value),
            z: CachedValue::new(z, number_to_value),
            angle: CachedValue::new(angle, angle_to_value),
        }
    }

    pub fn x(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value { rb_self.x.get(ruby) }
    pub fn y(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value { rb_self.y.get(ruby) }
    pub fn z(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value { rb_self.z.get(ruby) }
    pub fn angle(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value { rb_self.angle.get(ruby) }
}

impl DataTypeFunctions for YTransformRotate3D {
    fn mark(&self, marker: &gc::Marker) {
        self.x.mark(marker);
        self.y.mark(marker);
        self.z.mark(marker);
        self.angle.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Transform::Perspective", mark)]
pub struct YTransformPerspective {
    value: CachedValue<PerspectiveFunction<Length>>,
}

impl YTransformPerspective {
    pub fn new(value: PerspectiveFunction<Length>) -> Self {
        Self {
            value: CachedValue::new(value, make_perspective_value),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value { rb_self.value.get(ruby) }
}

impl DataTypeFunctions for YTransformPerspective {
    fn mark(&self, marker: &gc::Marker) {
        self.value.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::Declarations::Transform::Perspective::None")]
pub struct YTransformPerspectiveNone {}

impl YTransformPerspectiveNone {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Transform::Perspective::Length", mark)]
pub struct YTransformPerspectiveLength {
    value: CachedValue<Length>,
}

impl YTransformPerspectiveLength {
    pub fn new(value: Length) -> Self {
        Self {
            value: CachedValue::new(value, |length, ruby| length_to_value(length, ruby)),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value { rb_self.value.get(ruby) }
}

impl DataTypeFunctions for YTransformPerspectiveLength {
    fn mark(&self, marker: &gc::Marker) {
        self.value.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Transform::InterpolateMatrix", mark)]
pub struct YTransformInterpolateMatrix {
    from_list: CachedValue<Transform>,
    to_list: CachedValue<Transform>,
    progress: CachedValue<Percentage>,
}

impl YTransformInterpolateMatrix {
    pub fn new(from_list: Transform, to_list: Transform, progress: Percentage) -> Self {
        Self {
            from_list: CachedValue::new(from_list, |transform, ruby| {
                YTransform::new(transform.clone()).into_value_with(ruby)
            }),

            to_list: CachedValue::new(to_list, |transform, ruby| {
                YTransform::new(transform.clone()).into_value_with(ruby)
            }),

            progress: CachedValue::new(progress, |percentage, ruby| {
                YPercentage::new(percentage.0).into_value_with(ruby)
            }),
        }
    }

    pub fn from_list(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.from_list.get(ruby)
    }

    pub fn to_list(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.to_list.get(ruby)
    }

    pub fn progress(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.progress.get(ruby)
    }
}

impl DataTypeFunctions for YTransformInterpolateMatrix {
    fn mark(&self, marker: &gc::Marker) {
        self.from_list.mark(marker);
        self.to_list.mark(marker);
        self.progress.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Transform::AccumulateMatrix", mark)]
pub struct YTransformAccumulateMatrix {
    from_list: CachedValue<Transform>,
    to_list: CachedValue<Transform>,
    count: Integer,
}

impl YTransformAccumulateMatrix {
    pub fn new(from_list: Transform, to_list: Transform, count: Integer) -> Self {
        Self {
            from_list: CachedValue::new(from_list, |transform, ruby| {
                YTransform::new(transform.clone()).into_value_with(ruby)
            }),

            to_list: CachedValue::new(to_list, |transform, ruby| {
                YTransform::new(transform.clone()).into_value_with(ruby)
            }),

            count,
        }
    }

    pub fn from_list(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.from_list.get(ruby)
    }

    pub fn to_list(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.to_list.get(ruby)
    }

    pub fn count(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> i32 {
        rb_self.count.value()
    }
}

impl DataTypeFunctions for YTransformAccumulateMatrix {
    fn mark(&self, marker: &gc::Marker) {
        self.from_list.mark(marker);
        self.to_list.mark(marker);
    }
}
