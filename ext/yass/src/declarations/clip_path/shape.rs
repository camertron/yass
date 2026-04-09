use style::values::generics::{Optional, basic_shape::Shape};

use super::*;

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::ClipPath::ShapeFunction", mark)]
pub struct YClipPathShapeFunction {
    fill: FillRule,
    commands: CachedValueList<ClipPathShapeCommand>,
}

impl YClipPathShapeFunction {
    pub fn new(shape: Shape<Angle, ClipPathPosition, LengthPercentage>) -> Self {
        Self {
            fill: shape.fill,

            commands: CachedValueList::new(shape.commands.to_vec(), |command, _ctx, ruby| {
                make_shape_command(command, ruby)
            }),
        }
    }

    pub fn fill(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        fill_rule_to_id(rb_self.fill, ruby)
    }

    pub fn commands(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        rb_self.commands.to_a(ruby)
    }
}

impl DataTypeFunctions for YClipPathShapeFunction {
    fn mark(&self, marker: &gc::Marker) {
        self.commands.mark(marker);
    }
}

fn make_shape_command_value(command: &ClipPathShapeCommand, ruby: &Ruby) -> Value {
    match command {
        ClipPathShapeCommand::Move { point } => {
            YClipPathShapeCommandMove::new(point.clone()).into_value_with(ruby)
        }

        ClipPathShapeCommand::Line { point } => {
            YClipPathShapeCommandLine::new(point.clone()).into_value_with(ruby)
        }

        ClipPathShapeCommand::HLine { x } => {
            YClipPathShapeCommandHLine::new(x.clone()).into_value_with(ruby)
        }

        ClipPathShapeCommand::VLine { y } => {
            YClipPathShapeCommandVLine::new(y.clone()).into_value_with(ruby)
        }

        ClipPathShapeCommand::CubicCurve { control1, control2, point } => {
            let curve = YClipPathShapeCommandCubicCurve::new(
                control1.clone(),
                control2.clone(),
                point.clone(),
            );

            curve.into_value_with(ruby)
        }

        ClipPathShapeCommand::QuadCurve { control1, point } => {
            YClipPathShapeCommandQuadCurve::new(control1.clone(), point.clone()).into_value_with(ruby)
        }

        ClipPathShapeCommand::SmoothCubic { control2, point } => {
            YClipPathShapeCommandSmoothCubic::new(control2.clone(), point.clone()).into_value_with(ruby)
        }

        ClipPathShapeCommand::SmoothQuad { point } => {
            YClipPathShapeCommandSmoothQuad::new(point.clone()).into_value_with(ruby)
        }

        ClipPathShapeCommand::Arc { radii, arc_sweep, arc_size, rotate, point } => {
            let arc = YClipPathShapeCommandArc::new(
                radii.clone(),
                *arc_sweep,
                *arc_size,
                *rotate,
                point.clone(),
            );

            arc.into_value_with(ruby)
        }

        ClipPathShapeCommand::Close => YClipPathShapeCommandClose::new().into_value_with(ruby),
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::ClipPath::ShapeCommand", mark)]
pub struct YClipPathShapeCommand {
    value: CachedValue<ClipPathShapeCommand>,
}

impl YClipPathShapeCommand {
    pub fn new(command: ClipPathShapeCommand) -> Self {
        Self {
            value: CachedValue::new(command, make_shape_command_value),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.value.get(ruby)
    }
}

impl DataTypeFunctions for YClipPathShapeCommand {
    fn mark(&self, marker: &gc::Marker) {
        self.value.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::ClipPath::ShapeCommandMove", mark)]
pub struct YClipPathShapeCommandMove {
    point: CachedValue<ClipPathCommandEndPoint>,
}

impl YClipPathShapeCommandMove {
    pub fn new(point: ClipPathCommandEndPoint) -> Self {
        Self {
            point: CachedValue::new(point, make_command_end_point),
        }
    }

    pub fn point(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.point.get(ruby)
    }
}

impl DataTypeFunctions for YClipPathShapeCommandMove {
    fn mark(&self, marker: &gc::Marker) {
        self.point.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::ClipPath::ShapeCommandLine", mark)]
pub struct YClipPathShapeCommandLine {
    point: CachedValue<ClipPathCommandEndPoint>,
}

impl YClipPathShapeCommandLine {
    pub fn new(point: ClipPathCommandEndPoint) -> Self {
        Self {
            point: CachedValue::new(point, make_command_end_point),
        }
    }

    pub fn point(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.point.get(ruby)
    }
}

impl DataTypeFunctions for YClipPathShapeCommandLine {
    fn mark(&self, marker: &gc::Marker) {
        self.point.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::ClipPath::ShapeCommandHLine", mark)]
pub struct YClipPathShapeCommandHLine {
    x: CachedValue<ClipPathAxisEndPoint>,
}

impl YClipPathShapeCommandHLine {
    pub fn new(x: ClipPathAxisEndPoint) -> Self {
        Self {
            x: CachedValue::new(x, make_axis_end_point),
        }
    }

    pub fn x(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.x.get(ruby)
    }
}

impl DataTypeFunctions for YClipPathShapeCommandHLine {
    fn mark(&self, marker: &gc::Marker) {
        self.x.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::ClipPath::ShapeCommandVLine", mark)]
pub struct YClipPathShapeCommandVLine {
    y: CachedValue<ClipPathAxisEndPoint>,
}

impl YClipPathShapeCommandVLine {
    pub fn new(y: ClipPathAxisEndPoint) -> Self {
        Self {
            y: CachedValue::new(y, make_axis_end_point),
        }
    }

    pub fn y(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.y.get(ruby)
    }
}

impl DataTypeFunctions for YClipPathShapeCommandVLine {
    fn mark(&self, marker: &gc::Marker) {
        self.y.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::ClipPath::ShapeCommandCubicCurve", mark)]
pub struct YClipPathShapeCommandCubicCurve {
    control1: CachedValue<ClipPathControlPoint>,
    control2: CachedValue<ClipPathControlPoint>,
    point: CachedValue<ClipPathCommandEndPoint>,
}

impl YClipPathShapeCommandCubicCurve {
    pub fn new(
        control1: ClipPathControlPoint,
        control2: ClipPathControlPoint,
        point: ClipPathCommandEndPoint,
    ) -> Self {
        Self {
            control1: CachedValue::new(control1, make_control_point),
            control2: CachedValue::new(control2, make_control_point),
            point: CachedValue::new(point, make_command_end_point),
        }
    }

    pub fn control1(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.control1.get(ruby)
    }

    pub fn control2(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.control2.get(ruby)
    }

    pub fn point(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.point.get(ruby)
    }
}

impl DataTypeFunctions for YClipPathShapeCommandCubicCurve {
    fn mark(&self, marker: &gc::Marker) {
        self.control1.mark(marker);
        self.control2.mark(marker);
        self.point.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::ClipPath::ShapeCommandQuadCurve", mark)]
pub struct YClipPathShapeCommandQuadCurve {
    control1: CachedValue<ClipPathControlPoint>,
    point: CachedValue<ClipPathCommandEndPoint>,
}

impl YClipPathShapeCommandQuadCurve {
    pub fn new(control1: ClipPathControlPoint, point: ClipPathCommandEndPoint) -> Self {
        Self {
            control1: CachedValue::new(control1, make_control_point),
            point: CachedValue::new(point, make_command_end_point),
        }
    }

    pub fn control1(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.control1.get(ruby)
    }

    pub fn point(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.point.get(ruby)
    }
}

impl DataTypeFunctions for YClipPathShapeCommandQuadCurve {
    fn mark(&self, marker: &gc::Marker) {
        self.control1.mark(marker);
        self.point.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::ClipPath::ShapeCommandSmoothCubic", mark)]
pub struct YClipPathShapeCommandSmoothCubic {
    control2: CachedValue<ClipPathControlPoint>,
    point: CachedValue<ClipPathCommandEndPoint>,
}

impl YClipPathShapeCommandSmoothCubic {
    pub fn new(control2: ClipPathControlPoint, point: ClipPathCommandEndPoint) -> Self {
        Self {
            control2: CachedValue::new(control2, make_control_point),
            point: CachedValue::new(point, make_command_end_point),
        }
    }

    pub fn control2(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.control2.get(ruby)
    }

    pub fn point(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.point.get(ruby)
    }
}

impl DataTypeFunctions for YClipPathShapeCommandSmoothCubic {
    fn mark(&self, marker: &gc::Marker) {
        self.control2.mark(marker);
        self.point.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::ClipPath::ShapeCommandSmoothQuad", mark)]
pub struct YClipPathShapeCommandSmoothQuad {
    point: CachedValue<ClipPathCommandEndPoint>,
}

impl YClipPathShapeCommandSmoothQuad {
    pub fn new(point: ClipPathCommandEndPoint) -> Self {
        Self {
            point: CachedValue::new(point, make_command_end_point),
        }
    }

    pub fn point(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.point.get(ruby)
    }
}

impl DataTypeFunctions for YClipPathShapeCommandSmoothQuad {
    fn mark(&self, marker: &gc::Marker) {
        self.point.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::ClipPath::ShapeCommandArc", mark)]
pub struct YClipPathShapeCommandArc {
    radii: CachedValue<ArcRadii<LengthPercentage>>,
    arc_sweep: ArcSweep,
    arc_size: ArcSize,
    rotate: CachedValue<Angle>,
    point: CachedValue<ClipPathCommandEndPoint>,
}

impl YClipPathShapeCommandArc {
    pub fn new(
        radii: ArcRadii<LengthPercentage>,
        arc_sweep: ArcSweep,
        arc_size: ArcSize,
        rotate: Angle,
        point: ClipPathCommandEndPoint,
    ) -> Self {
        Self {
            radii: CachedValue::new(radii, make_arc_radii),
            arc_sweep,
            arc_size,
            rotate: CachedValue::new(rotate, |rotate, ruby| {
                YAngle::new(rotate.clone()).into_value_with(ruby)
            }),
            point: CachedValue::new(point, make_command_end_point),
        }
    }

    pub fn radii(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.radii.get(ruby)
    }

    pub fn arc_sweep(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        arc_sweep_to_id(rb_self.arc_sweep, ruby)
    }

    pub fn arc_size(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        arc_size_to_id(rb_self.arc_size, ruby)
    }

    pub fn rotate(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.rotate.get(ruby)
    }

    pub fn point(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.point.get(ruby)
    }
}

impl DataTypeFunctions for YClipPathShapeCommandArc {
    fn mark(&self, marker: &gc::Marker) {
        self.radii.mark(marker);
        self.rotate.mark(marker);
        self.point.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::Declarations::ClipPath::ShapeCommandClose")]
pub struct YClipPathShapeCommandClose {}

impl YClipPathShapeCommandClose {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::ClipPath::CommandEndPointToPosition", mark)]
pub struct YClipPathCommandEndPointToPosition {
    horizontal: CachedValue<PositionComponent<HorizontalPositionKeyword>>,
    vertical: CachedValue<PositionComponent<VerticalPositionKeyword>>,
}

impl YClipPathCommandEndPointToPosition {
    pub fn new(position: ClipPathPosition) -> Self {
        Self {
            horizontal: CachedValue::new(position.horizontal, |component, ruby| {
                make_horizontal_position_component(component.clone(), ruby)
            }),

            vertical: CachedValue::new(position.vertical, |component, ruby| {
                make_vertical_position_component(component.clone(), ruby)
            }),
        }
    }

    pub fn horizontal(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.horizontal.get(ruby)
    }

    pub fn vertical(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.vertical.get(ruby)
    }
}

impl DataTypeFunctions for YClipPathCommandEndPointToPosition {
    fn mark(&self, marker: &gc::Marker) {
        self.horizontal.mark(marker);
        self.vertical.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::ClipPath::CommandEndPointByCoordinate", mark)]
pub struct YClipPathCommandEndPointByCoordinate {
    coord: CachedValue<CoordinatePair<LengthPercentage>>,
}

impl YClipPathCommandEndPointByCoordinate {
    pub fn new(coord: CoordinatePair<LengthPercentage>) -> Self {
        Self {
            coord: CachedValue::new(coord, make_coordinate_pair),
        }
    }

    pub fn coord(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.coord.get(ruby)
    }
}

impl DataTypeFunctions for YClipPathCommandEndPointByCoordinate {
    fn mark(&self, marker: &gc::Marker) {
        self.coord.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::ClipPath::CoordinatePair", mark)]
pub struct YClipPathCoordinatePair {
    x: CachedValue<LengthPercentage>,
    y: CachedValue<LengthPercentage>,
}

impl YClipPathCoordinatePair {
    pub fn new(coord: CoordinatePair<LengthPercentage>) -> Self {
        Self {
            x: CachedValue::new(coord.x, |x, ruby| length_percentage_to_value(x, ruby)),
            y: CachedValue::new(coord.y, |y, ruby| length_percentage_to_value(y, ruby)),
        }
    }

    pub fn x(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.x.get(ruby)
    }

    pub fn y(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.y.get(ruby)
    }
}

impl DataTypeFunctions for YClipPathCoordinatePair {
    fn mark(&self, marker: &gc::Marker) {
        self.x.mark(marker);
        self.y.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::Declarations::ClipPath::AxisEndPointToPosition")]
pub struct YClipPathAxisEndPointToPosition {
    position: AxisPosition<LengthPercentage>,
}

impl YClipPathAxisEndPointToPosition {
    pub fn new(position: AxisPosition<LengthPercentage>) -> Self {
        Self { position }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        match &rb_self.position {
            AxisPosition::LengthPercent(length) => {
                length_percentage_to_value(length, ruby)
            }

            AxisPosition::Keyword(keyword) => {
                axis_position_keyword_to_id(*keyword, ruby).into_value_with(ruby)
            }
        }
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::ClipPath::AxisEndPointByCoordinate", mark)]
pub struct YClipPathAxisEndPointByCoordinate {
    coord: CachedValue<LengthPercentage>,
}

impl YClipPathAxisEndPointByCoordinate {
    pub fn new(coord: LengthPercentage) -> Self {
        Self {
            coord: CachedValue::new(coord, |coord, ruby| {
                length_percentage_to_value(coord, ruby)
            }),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.coord.get(ruby)
    }
}

impl DataTypeFunctions for YClipPathAxisEndPointByCoordinate {
    fn mark(&self, marker: &gc::Marker) {
        self.coord.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::ClipPath::ControlPointAbsolute", mark)]
pub struct YClipPathControlPointAbsolute {
    position: CachedValue<ClipPathPosition>,
}

impl YClipPathControlPointAbsolute {
    pub fn new(position: ClipPathPosition) -> Self {
        Self {
            position: CachedValue::new(position, |position, ruby| {
                YClipPathCommandEndPointToPosition::new(position.clone()).into_value_with(ruby)
            }),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.position.get(ruby)
    }
}

impl DataTypeFunctions for YClipPathControlPointAbsolute {
    fn mark(&self, marker: &gc::Marker) {
        self.position.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::ClipPath::ControlPointRelative", mark)]
pub struct YClipPathControlPointRelative {
    relative: CachedValue<RelativeControlPoint<LengthPercentage>>,
}

impl YClipPathControlPointRelative {
    pub fn new(relative: RelativeControlPoint<LengthPercentage>) -> Self {
        Self {
            relative: CachedValue::new(relative, |relative, ruby| {
                YClipPathRelativeControlPoint::new(relative.clone()).into_value_with(ruby)
            }),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.relative.get(ruby)
    }
}

impl DataTypeFunctions for YClipPathControlPointRelative {
    fn mark(&self, marker: &gc::Marker) {
        self.relative.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::ClipPath::RelativeControlPoint", mark)]
pub struct YClipPathRelativeControlPoint {
    coord: CachedValue<CoordinatePair<LengthPercentage>>,
    reference: ControlReference,
}

impl YClipPathRelativeControlPoint {
    pub fn new(relative: RelativeControlPoint<LengthPercentage>) -> Self {
        Self {
            coord: CachedValue::new(relative.coord, make_coordinate_pair),
            reference: relative.reference,
        }
    }

    pub fn coord(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.coord.get(ruby)
    }

    pub fn reference(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.reference {
            ControlReference::Start => ruby.intern("start"),
            ControlReference::End => ruby.intern("end"),
            ControlReference::Origin => ruby.intern("origin"),
        }
    }
}

impl DataTypeFunctions for YClipPathRelativeControlPoint {
    fn mark(&self, marker: &gc::Marker) {
        self.coord.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::ClipPath::ArcRadii", mark)]
pub struct YClipPathArcRadii {
    rx: CachedValue<LengthPercentage>,
    ry: CachedValue<Optional<LengthPercentage>>,
}

impl YClipPathArcRadii {
    pub fn new(radii: ArcRadii<LengthPercentage>) -> Self {
        Self {
            rx: CachedValue::new(radii.rx, |rx, ruby| length_percentage_to_value(rx, ruby)),
            ry: CachedValue::new(radii.ry, |ry, ruby| {
                match ry {
                    Optional::Some(value) => {
                        length_percentage_to_value(value, ruby)
                    }

                    Optional::None => ruby.qnil().into_value_with(ruby),
                }
            }),
        }
    }

    pub fn rx(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.rx.get(ruby)
    }

    pub fn ry(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.ry.get(ruby)
    }
}

impl DataTypeFunctions for YClipPathArcRadii {
    fn mark(&self, marker: &gc::Marker) {
        self.rx.mark(marker);
        self.ry.mark(marker);
    }
}
