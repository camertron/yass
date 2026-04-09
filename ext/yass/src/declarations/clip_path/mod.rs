use magnus::{
    DataTypeFunctions, Error, IntoValue, RArray, Ruby, TypedData, Value, gc, typed_data,
    value::Id,
};
use style::values::{
    CssUrl,
    generics::{
        NonNegative,
        basic_shape::{
            ArcRadii, ArcSize, ArcSweep, AxisEndPoint, AxisPosition, AxisPositionKeyword,
            BasicShape as StyloBasicShape, ClipPath as StyloClipPath, CommandEndPoint,
            ControlPoint, ControlReference, CoordinatePair, FillRule, PolygonCoord,
            RelativeControlPoint, ShapeBox, ShapeGeometryBox, ShapeRadius as GenericShapeRadius,
        },
        border::{BorderCornerRadius, BorderRadius as GenericBorderRadius},
        position::{Position, PositionOrAuto as GenericPositionOrAuto}
    },
    specified::{
        Angle,
        LengthPercentage,
        PositionComponent,
        basic_shape::{BasicShapeRect, PathOrShapeFunction as GenericPathOrShapeFunction, ShapeRectFunction, Xywh},
        length::LengthPercentageOrAuto,
        position::{HorizontalPositionKeyword, VerticalPositionKeyword},
        svg_path::PathCommand,
    },
};
use style_traits::ToCss;

use crate::{
    cached_value::CachedValue,
    cached_value_list::CachedValueList,
    declarations::{
        angle::YAngle,
        horizontal_position_component::make_horizontal_position_component,
        number::YNumber,
        size::{YLengthPercentage, length_percentage_to_value},
        vertical_position_component::make_vertical_position_component,
    },
};

pub mod circle_ellipse;
pub mod init;
pub mod path;
pub mod rect;
pub mod shape;
pub mod shape_box;

use self::{
    circle_ellipse::{
        YClipPathAuto, YClipPathCircle, YClipPathEllipse, YClipPathPosition,
        YClipPathPositionAuto, YClipPathShapeRadiusClosestSide, YClipPathShapeRadiusFarthestSide,
        YClipPathShapeRadiusLength,
    },
    path::{
        YClipPathPathCommand, YClipPathPathFunction, YClipPathPathOrShape, YClipPathPolygon,
    },
    rect::{
        YClipPathBorderCornerRadius, YClipPathBorderRadius, YClipPathInsetRect,
        YClipPathRect, YClipPathRectFunction, YClipPathXywhRect,
    },
    shape::{
        YClipPathArcRadii,
        YClipPathAxisEndPointByCoordinate,
        YClipPathAxisEndPointToPosition,
        YClipPathCommandEndPointByCoordinate,
        YClipPathCommandEndPointToPosition,
        YClipPathCoordinatePair,
        YClipPathControlPointAbsolute,
        YClipPathControlPointRelative,
        YClipPathShapeCommand,
        YClipPathShapeFunction,
    },
    shape_box::{
        YClipPathBorderBox, YClipPathContentBox, YClipPathElementDependent, YClipPathFillBox,
        YClipPathMarginBox, YClipPathPaddingBox, YClipPathStrokeBox, YClipPathViewBox,
    },
};

type ClipPathPosition = Position<
    PositionComponent<HorizontalPositionKeyword>,
    PositionComponent<VerticalPositionKeyword>,
>;

type ClipPathBasicShape =
    StyloBasicShape<Angle, ClipPathPosition, LengthPercentage, BasicShapeRect>;

type ClipPath = StyloClipPath<ClipPathBasicShape, CssUrl>;

type ClipPathPathOrShape = GenericPathOrShapeFunction;

type ClipPathShapePosition = style::values::generics::basic_shape::ShapePosition<LengthPercentage>;

type ClipPathPositionOrAuto = GenericPositionOrAuto<ClipPathShapePosition>;

type ClipPathShapeRadius = GenericShapeRadius<LengthPercentage>;

type ClipPathShapeCommand =
    style::values::generics::basic_shape::ShapeCommand<Angle, ClipPathPosition, LengthPercentage>;

type ClipPathBorderRadius = GenericBorderRadius<NonNegative<LengthPercentage>>;

type ClipPathBorderCornerRadius = BorderCornerRadius<NonNegative<LengthPercentage>>;

type ClipPathCommandEndPoint = CommandEndPoint<ClipPathPosition, LengthPercentage>;

type ClipPathAxisEndPoint = AxisEndPoint<LengthPercentage>;

type ClipPathControlPoint = ControlPoint<ClipPathPosition, LengthPercentage>;

type ClipPathSvgPosition = style::values::generics::basic_shape::ShapePosition<style::values::CSSFloat>;

type ClipPathPathCommandEndPoint = CommandEndPoint<ClipPathSvgPosition, style::values::CSSFloat>;

type ClipPathPathAxisEndPoint = AxisEndPoint<style::values::CSSFloat>;

type ClipPathPathControlPoint = ControlPoint<ClipPathSvgPosition, style::values::CSSFloat>;

type ClipPathPathArcRadii = ArcRadii<style::values::CSSFloat>;

fn fill_rule_to_id(fill_rule: FillRule, ruby: &Ruby) -> Id {
    match fill_rule {
        FillRule::Nonzero => ruby.intern("nonzero"),
        FillRule::Evenodd => ruby.intern("evenodd"),
    }
}

fn make_length_percentage_or_auto(value: &LengthPercentageOrAuto, ruby: &Ruby) -> Value {
    match value {
        LengthPercentageOrAuto::Auto => YClipPathAuto::new().into_value_with(ruby),
        LengthPercentageOrAuto::LengthPercentage(length_percentage) => {
            length_percentage_to_value(length_percentage, ruby)
        }
    }
}

fn make_shape_radius(shape_radius: &ClipPathShapeRadius, ruby: &Ruby) -> Value {
    match shape_radius {
        ClipPathShapeRadius::Length(length_percentage) => {
            YClipPathShapeRadiusLength::new(length_percentage.clone()).into_value_with(ruby)
        }
        ClipPathShapeRadius::ClosestSide => {
            YClipPathShapeRadiusClosestSide::new().into_value_with(ruby)
        }
        ClipPathShapeRadius::FarthestSide => {
            YClipPathShapeRadiusFarthestSide::new().into_value_with(ruby)
        }
    }
}

fn make_shape_position_or_auto(position: &ClipPathPositionOrAuto, ruby: &Ruby) -> Value {
    match position {
        ClipPathPositionOrAuto::Auto => YClipPathPositionAuto::new().into_value_with(ruby),
        ClipPathPositionOrAuto::Position(position) => {
            YClipPathPosition::new(position.clone()).into_value_with(ruby)
        }
    }
}

fn make_border_corner_radius(corner: &ClipPathBorderCornerRadius, ruby: &Ruby) -> Value {
    YClipPathBorderCornerRadius::new(corner.clone()).into_value_with(ruby)
}

fn make_border_radius(radius: &ClipPathBorderRadius, ruby: &Ruby) -> Value {
    YClipPathBorderRadius::new(radius.clone()).into_value_with(ruby)
}

fn axis_position_keyword_to_id(keyword: AxisPositionKeyword, ruby: &Ruby) -> Id {
    match keyword {
        AxisPositionKeyword::Center => ruby.intern("center"),
        AxisPositionKeyword::Left => ruby.intern("left"),
        AxisPositionKeyword::Right => ruby.intern("right"),
        AxisPositionKeyword::Top => ruby.intern("top"),
        AxisPositionKeyword::Bottom => ruby.intern("bottom"),
        AxisPositionKeyword::XStart => ruby.intern("x_start"),
        AxisPositionKeyword::XEnd => ruby.intern("x_end"),
        AxisPositionKeyword::YStart => ruby.intern("y_start"),
        AxisPositionKeyword::YEnd => ruby.intern("y_end"),
    }
}

fn make_coordinate_pair(coord: &CoordinatePair<LengthPercentage>, ruby: &Ruby) -> Value {
    YClipPathCoordinatePair::new(coord.clone()).into_value_with(ruby)
}

fn make_command_end_point(point: &ClipPathCommandEndPoint, ruby: &Ruby) -> Value {
    match point {
        CommandEndPoint::ToPosition(position) => {
            YClipPathCommandEndPointToPosition::new(position.clone()).into_value_with(ruby)
        }
        CommandEndPoint::ByCoordinate(coord) => {
            YClipPathCommandEndPointByCoordinate::new(coord.clone()).into_value_with(ruby)
        }
    }
}

fn make_axis_end_point(point: &ClipPathAxisEndPoint, ruby: &Ruby) -> Value {
    match point {
        AxisEndPoint::ToPosition(position) => {
            YClipPathAxisEndPointToPosition::new(position.clone()).into_value_with(ruby)
        }
        AxisEndPoint::ByCoordinate(coord) => {
            YClipPathAxisEndPointByCoordinate::new(coord.clone()).into_value_with(ruby)
        }
    }
}

fn make_control_point(point: &ClipPathControlPoint, ruby: &Ruby) -> Value {
    match point {
        ControlPoint::Absolute(position) => {
            YClipPathControlPointAbsolute::new(position.clone()).into_value_with(ruby)
        }
        ControlPoint::Relative(relative) => {
            YClipPathControlPointRelative::new(relative.clone()).into_value_with(ruby)
        }
    }
}

fn make_arc_radii(radii: &ArcRadii<LengthPercentage>, ruby: &Ruby) -> Value {
    YClipPathArcRadii::new(radii.clone()).into_value_with(ruby)
}

fn arc_sweep_to_id(arc_sweep: ArcSweep, ruby: &Ruby) -> Id {
    match arc_sweep {
        ArcSweep::Ccw => ruby.intern("ccw"),
        ArcSweep::Cw => ruby.intern("cw"),
    }
}

fn arc_size_to_id(arc_size: ArcSize, ruby: &Ruby) -> Id {
    match arc_size {
        ArcSize::Small => ruby.intern("small"),
        ArcSize::Large => ruby.intern("large"),
    }
}

fn make_shape_command(shape_command: &ClipPathShapeCommand, ruby: &Ruby) -> Value {
    YClipPathShapeCommand::new(shape_command.clone()).into_value_with(ruby)
}

fn make_path_command(path_command: &PathCommand, ruby: &Ruby) -> Value {
    YClipPathPathCommand::new(*path_command).into_value_with(ruby)
}

fn make_shape_box(shape_box: &ShapeBox, ruby: &Ruby) -> Value {
    match shape_box {
        ShapeBox::MarginBox => YClipPathMarginBox::new().into_value_with(ruby),
        ShapeBox::BorderBox => YClipPathBorderBox::new().into_value_with(ruby),
        ShapeBox::PaddingBox => YClipPathPaddingBox::new().into_value_with(ruby),
        ShapeBox::ContentBox => YClipPathContentBox::new().into_value_with(ruby),
    }
}

fn make_shape_geometry_box(reference_box: &ShapeGeometryBox, ruby: &Ruby) -> Value {
    match reference_box {
        ShapeGeometryBox::ElementDependent => {
            YClipPathElementDependent::new().into_value_with(ruby)
        }
        ShapeGeometryBox::FillBox => YClipPathFillBox::new().into_value_with(ruby),
        ShapeGeometryBox::StrokeBox => YClipPathStrokeBox::new().into_value_with(ruby),
        ShapeGeometryBox::ViewBox => YClipPathViewBox::new().into_value_with(ruby),
        ShapeGeometryBox::ShapeBox(shape_box) => make_shape_box(shape_box, ruby),
    }
}

fn make_basic_shape_rect(rect: &BasicShapeRect, ruby: &Ruby) -> Value {
    match rect {
        BasicShapeRect::Inset(inset_rect) => {
            YClipPathInsetRect::new(inset_rect.clone()).into_value_with(ruby)
        }
        BasicShapeRect::Xywh(xywh) => YClipPathXywhRect::new(xywh.clone()).into_value_with(ruby),
        BasicShapeRect::Rect(shape_rect) => {
            YClipPathRectFunction::new(shape_rect.clone()).into_value_with(ruby)
        }
    }
}

fn make_basic_shape_path_or_shape(path_or_shape: &ClipPathPathOrShape, ruby: &Ruby) -> Value {
    match path_or_shape {
        ClipPathPathOrShape::Path(path) => {
            YClipPathPathFunction::new(path.clone()).into_value_with(ruby)
        }
        ClipPathPathOrShape::Shape(shape) => {
            YClipPathShapeFunction::new(shape.clone()).into_value_with(ruby)
        }
    }
}

fn make_basic_shape(basic_shape: &ClipPathBasicShape, ruby: &Ruby) -> Value {
    match basic_shape {
        ClipPathBasicShape::Rect(rect) => YClipPathRect::new(rect.clone()).into_value_with(ruby),
        ClipPathBasicShape::Circle(circle) => {
            YClipPathCircle::new(circle.clone()).into_value_with(ruby)
        }
        ClipPathBasicShape::Ellipse(ellipse) => {
            YClipPathEllipse::new(ellipse.clone()).into_value_with(ruby)
        }
        ClipPathBasicShape::Polygon(polygon) => {
            YClipPathPolygon::new(polygon.clone()).into_value_with(ruby)
        }
        ClipPathBasicShape::PathOrShape(path_or_shape) => {
            YClipPathPathOrShape::new(path_or_shape.clone()).into_value_with(ruby)
        }
    }
}

fn make_clip_path(clip_path: &ClipPath, ruby: &Ruby) -> Value {
    match clip_path {
        ClipPath::None => YClipPathNone::new().into_value_with(ruby),
        ClipPath::Url(url) => YClipPathUrl::new(url.to_css_string()).into_value_with(ruby),
        ClipPath::Shape(basic_shape, reference_box) => {
            YClipPathShape::new((**basic_shape).clone(), *reference_box).into_value_with(ruby)
        }
        ClipPath::Box(reference_box) => YClipPathBox::new(*reference_box).into_value_with(ruby),
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::ClipPath", mark)]
pub struct YClipPath {
    clip_path: CachedValue<ClipPath>,
}

impl YClipPath {
    pub fn new(clip_path: ClipPath) -> Self {
        Self {
            clip_path: CachedValue::new(clip_path, make_clip_path),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.clip_path.get(ruby)
    }
}

impl DataTypeFunctions for YClipPath {
    fn mark(&self, marker: &gc::Marker) {
        self.clip_path.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::Declarations::ClipPath::None")]
pub struct YClipPathNone {}

impl YClipPathNone {
    pub fn new() -> Self {
        Self {}
    }
}

#[magnus::wrap(class = "Yass::Declarations::ClipPath::Url")]
pub struct YClipPathUrl {
    css: String,
}

impl YClipPathUrl {
    pub fn new(css: String) -> Self {
        Self { css }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        ruby.str_new(&rb_self.css).into_value_with(ruby)
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::ClipPath::Shape", mark)]
pub struct YClipPathShape {
    basic_shape: CachedValue<ClipPathBasicShape>,
    reference_box: CachedValue<ShapeGeometryBox>,
}

impl YClipPathShape {
    pub fn new(basic_shape: ClipPathBasicShape, reference_box: ShapeGeometryBox) -> Self {
        Self {
            basic_shape: CachedValue::new(basic_shape, make_basic_shape),
            reference_box: CachedValue::new(reference_box, make_shape_geometry_box),
        }
    }

    pub fn shape(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.basic_shape.get(ruby)
    }

    pub fn reference_box(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.reference_box.get(ruby)
    }
}

impl DataTypeFunctions for YClipPathShape {
    fn mark(&self, marker: &gc::Marker) {
        self.basic_shape.mark(marker);
        self.reference_box.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::ClipPath::Box", mark)]
pub struct YClipPathBox {
    reference_box: CachedValue<ShapeGeometryBox>,
}

impl YClipPathBox {
    pub fn new(reference_box: ShapeGeometryBox) -> Self {
        Self {
            reference_box: CachedValue::new(reference_box, make_shape_geometry_box),
        }
    }

    pub fn reference_box(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.reference_box.get(ruby)
    }
}

impl DataTypeFunctions for YClipPathBox {
    fn mark(&self, marker: &gc::Marker) {
        self.reference_box.mark(marker);
    }
}
