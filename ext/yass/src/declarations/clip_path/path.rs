use super::*;

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::ClipPath::Polygon", mark)]
pub struct YClipPathPolygon {
    fill: FillRule,
    coordinates: CachedValueList<PolygonCoord<LengthPercentage>>,
}

impl YClipPathPolygon {
    pub fn new(polygon: style::values::generics::basic_shape::Polygon<LengthPercentage>) -> Self {
        Self {
            fill: polygon.fill,
            coordinates: CachedValueList::new(polygon.coordinates.to_vec(), |coord, _ctx, ruby| {
                YClipPathPolygonCoord::new(coord.clone()).into_value_with(ruby)
            }),
        }
    }

    pub fn fill(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        fill_rule_to_id(rb_self.fill, ruby)
    }

    pub fn coordinates(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        rb_self.coordinates.to_a(ruby)
    }
}

impl DataTypeFunctions for YClipPathPolygon {
    fn mark(&self, marker: &gc::Marker) {
        self.coordinates.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::ClipPath::PolygonCoord", mark)]
pub struct YClipPathPolygonCoord {
    x: CachedValue<LengthPercentage>,
    y: CachedValue<LengthPercentage>,
}

impl YClipPathPolygonCoord {
    pub fn new(coord: PolygonCoord<LengthPercentage>) -> Self {
        Self {
            x: CachedValue::new(coord.0, |value, ruby| {
                length_percentage_to_value(value.clone(), ruby)
            }),

            y: CachedValue::new(coord.1, |value, ruby| {
                length_percentage_to_value(value.clone(), ruby)
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

impl DataTypeFunctions for YClipPathPolygonCoord {
    fn mark(&self, marker: &gc::Marker) {
        self.x.mark(marker);
        self.y.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::ClipPath::PathOrShape", mark)]
pub struct YClipPathPathOrShape {
    value: CachedValue<ClipPathPathOrShape>,
}

impl YClipPathPathOrShape {
    pub fn new(path_or_shape: ClipPathPathOrShape) -> Self {
        Self {
            value: CachedValue::new(path_or_shape, make_basic_shape_path_or_shape),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.value.get(ruby)
    }
}

impl DataTypeFunctions for YClipPathPathOrShape {
    fn mark(&self, marker: &gc::Marker) {
        self.value.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::ClipPath::PathFunction", mark)]
pub struct YClipPathPathFunction {
    fill: FillRule,
    commands: CachedValueList<PathCommand>,
}

impl YClipPathPathFunction {
    pub fn new(path: style::values::generics::basic_shape::Path) -> Self {
        Self {
            fill: path.fill,
            commands: CachedValueList::new(path.path.commands().to_vec(), |command, _ctx, ruby| {
                make_path_command(command, ruby)
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

impl DataTypeFunctions for YClipPathPathFunction {
    fn mark(&self, marker: &gc::Marker) {
        self.commands.mark(marker);
    }
}

fn make_path_command_value(command: &PathCommand, ruby: &Ruby) -> Value {
    match command {
        PathCommand::Move { point } => {
            YClipPathPathCommandMove::new(*point).into_value_with(ruby)
        }
        PathCommand::Line { point } => {
            YClipPathPathCommandLine::new(*point).into_value_with(ruby)
        }
        PathCommand::HLine { x } => {
            YClipPathPathCommandHLine::new(*x).into_value_with(ruby)
        }
        PathCommand::VLine { y } => {
            YClipPathPathCommandVLine::new(*y).into_value_with(ruby)
        }
        PathCommand::CubicCurve {
            control1,
            control2,
            point,
        } => YClipPathPathCommandCubicCurve::new(*control1, *control2, *point).into_value_with(ruby),
        PathCommand::QuadCurve { control1, point } => {
            YClipPathPathCommandQuadCurve::new(*control1, *point).into_value_with(ruby)
        }
        PathCommand::SmoothCubic { control2, point } => {
            YClipPathPathCommandSmoothCubic::new(*control2, *point).into_value_with(ruby)
        }
        PathCommand::SmoothQuad { point } => {
            YClipPathPathCommandSmoothQuad::new(*point).into_value_with(ruby)
        }
        PathCommand::Arc {
            radii,
            arc_sweep,
            arc_size,
            rotate,
            point,
        } => YClipPathPathCommandArc::new(*radii, *arc_sweep, *arc_size, *rotate, *point)
            .into_value_with(ruby),
        PathCommand::Close => YClipPathPathCommandClose::new().into_value_with(ruby),
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::ClipPath::PathCommand", mark)]
pub struct YClipPathPathCommand {
    value: CachedValue<PathCommand>,
}

impl YClipPathPathCommand {
    pub fn new(command: PathCommand) -> Self {
        Self {
            value: CachedValue::new(command, make_path_command_value),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.value.get(ruby)
    }
}

impl DataTypeFunctions for YClipPathPathCommand {
    fn mark(&self, marker: &gc::Marker) {
        self.value.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::ClipPath::PathCommandMove", mark)]
pub struct YClipPathPathCommandMove {
    point: CachedValue<ClipPathPathCommandEndPoint>,
}

impl YClipPathPathCommandMove {
    pub fn new(point: ClipPathPathCommandEndPoint) -> Self {
        Self {
            point: CachedValue::new(point, make_path_command_end_point),
        }
    }

    pub fn point(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.point.get(ruby)
    }
}

impl DataTypeFunctions for YClipPathPathCommandMove {
    fn mark(&self, marker: &gc::Marker) {
        self.point.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::ClipPath::PathCommandLine", mark)]
pub struct YClipPathPathCommandLine {
    point: CachedValue<ClipPathPathCommandEndPoint>,
}

impl YClipPathPathCommandLine {
    pub fn new(point: ClipPathPathCommandEndPoint) -> Self {
        Self {
            point: CachedValue::new(point, make_path_command_end_point),
        }
    }

    pub fn point(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.point.get(ruby)
    }
}

impl DataTypeFunctions for YClipPathPathCommandLine {
    fn mark(&self, marker: &gc::Marker) {
        self.point.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::ClipPath::PathCommandHLine", mark)]
pub struct YClipPathPathCommandHLine {
    x: CachedValue<ClipPathPathAxisEndPoint>,
}

impl YClipPathPathCommandHLine {
    pub fn new(x: ClipPathPathAxisEndPoint) -> Self {
        Self {
            x: CachedValue::new(x, make_path_axis_end_point),
        }
    }

    pub fn x(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.x.get(ruby)
    }
}

impl DataTypeFunctions for YClipPathPathCommandHLine {
    fn mark(&self, marker: &gc::Marker) {
        self.x.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::ClipPath::PathCommandVLine", mark)]
pub struct YClipPathPathCommandVLine {
    y: CachedValue<ClipPathPathAxisEndPoint>,
}

impl YClipPathPathCommandVLine {
    pub fn new(y: ClipPathPathAxisEndPoint) -> Self {
        Self {
            y: CachedValue::new(y, make_path_axis_end_point),
        }
    }

    pub fn y(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.y.get(ruby)
    }
}

impl DataTypeFunctions for YClipPathPathCommandVLine {
    fn mark(&self, marker: &gc::Marker) {
        self.y.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::ClipPath::PathCommandCubicCurve", mark)]
pub struct YClipPathPathCommandCubicCurve {
    control1: CachedValue<ClipPathPathControlPoint>,
    control2: CachedValue<ClipPathPathControlPoint>,
    point: CachedValue<ClipPathPathCommandEndPoint>,
}

impl YClipPathPathCommandCubicCurve {
    pub fn new(
        control1: ClipPathPathControlPoint,
        control2: ClipPathPathControlPoint,
        point: ClipPathPathCommandEndPoint,
    ) -> Self {
        Self {
            control1: CachedValue::new(control1, make_path_control_point),
            control2: CachedValue::new(control2, make_path_control_point),
            point: CachedValue::new(point, make_path_command_end_point),
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

impl DataTypeFunctions for YClipPathPathCommandCubicCurve {
    fn mark(&self, marker: &gc::Marker) {
        self.control1.mark(marker);
        self.control2.mark(marker);
        self.point.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::ClipPath::PathCommandQuadCurve", mark)]
pub struct YClipPathPathCommandQuadCurve {
    control1: CachedValue<ClipPathPathControlPoint>,
    point: CachedValue<ClipPathPathCommandEndPoint>,
}

impl YClipPathPathCommandQuadCurve {
    pub fn new(control1: ClipPathPathControlPoint, point: ClipPathPathCommandEndPoint) -> Self {
        Self {
            control1: CachedValue::new(control1, make_path_control_point),
            point: CachedValue::new(point, make_path_command_end_point),
        }
    }

    pub fn control1(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.control1.get(ruby)
    }

    pub fn point(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.point.get(ruby)
    }
}

impl DataTypeFunctions for YClipPathPathCommandQuadCurve {
    fn mark(&self, marker: &gc::Marker) {
        self.control1.mark(marker);
        self.point.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::ClipPath::PathCommandSmoothCubic", mark)]
pub struct YClipPathPathCommandSmoothCubic {
    control2: CachedValue<ClipPathPathControlPoint>,
    point: CachedValue<ClipPathPathCommandEndPoint>,
}

impl YClipPathPathCommandSmoothCubic {
    pub fn new(control2: ClipPathPathControlPoint, point: ClipPathPathCommandEndPoint) -> Self {
        Self {
            control2: CachedValue::new(control2, make_path_control_point),
            point: CachedValue::new(point, make_path_command_end_point),
        }
    }

    pub fn control2(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.control2.get(ruby)
    }

    pub fn point(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.point.get(ruby)
    }
}

impl DataTypeFunctions for YClipPathPathCommandSmoothCubic {
    fn mark(&self, marker: &gc::Marker) {
        self.control2.mark(marker);
        self.point.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::ClipPath::PathCommandSmoothQuad", mark)]
pub struct YClipPathPathCommandSmoothQuad {
    point: CachedValue<ClipPathPathCommandEndPoint>,
}

impl YClipPathPathCommandSmoothQuad {
    pub fn new(point: ClipPathPathCommandEndPoint) -> Self {
        Self {
            point: CachedValue::new(point, make_path_command_end_point),
        }
    }

    pub fn point(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.point.get(ruby)
    }
}

impl DataTypeFunctions for YClipPathPathCommandSmoothQuad {
    fn mark(&self, marker: &gc::Marker) {
        self.point.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::ClipPath::PathCommandArc", mark)]
pub struct YClipPathPathCommandArc {
    radii: CachedValue<ClipPathPathArcRadii>,
    arc_sweep: ArcSweep,
    arc_size: ArcSize,
    rotate: CachedValue<style::values::CSSFloat>,
    point: CachedValue<ClipPathPathCommandEndPoint>,
}

impl YClipPathPathCommandArc {
    pub fn new(
        radii: ClipPathPathArcRadii,
        arc_sweep: ArcSweep,
        arc_size: ArcSize,
        rotate: style::values::CSSFloat,
        point: ClipPathPathCommandEndPoint,
    ) -> Self {
        Self {
            radii: CachedValue::new(radii, make_path_arc_radii),
            arc_sweep,
            arc_size,
            rotate: CachedValue::new(rotate, |rotate, ruby| {
                YNumber::new(*rotate).into_value_with(ruby)
            }),
            point: CachedValue::new(point, make_path_command_end_point),
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

impl DataTypeFunctions for YClipPathPathCommandArc {
    fn mark(&self, marker: &gc::Marker) {
        self.radii.mark(marker);
        self.rotate.mark(marker);
        self.point.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::Declarations::ClipPath::PathCommandClose")]
pub struct YClipPathPathCommandClose {}

impl YClipPathPathCommandClose {
    pub fn new() -> Self {
        Self {}
    }
}

fn make_path_coordinate_pair(coord: &CoordinatePair<style::values::CSSFloat>, ruby: &Ruby) -> Value {
    YClipPathPathCoordinatePair::new(*coord).into_value_with(ruby)
}

fn make_path_command_end_point(point: &ClipPathPathCommandEndPoint, ruby: &Ruby) -> Value {
    match point {
        CommandEndPoint::ToPosition(position) => {
            YClipPathPathCommandEndPointToPosition::new(*position).into_value_with(ruby)
        }
        CommandEndPoint::ByCoordinate(coord) => {
            YClipPathPathCommandEndPointByCoordinate::new(*coord).into_value_with(ruby)
        }
    }
}

fn make_path_axis_end_point(point: &ClipPathPathAxisEndPoint, ruby: &Ruby) -> Value {
    match point {
        AxisEndPoint::ToPosition(position) => {
            YClipPathPathAxisEndPointToPosition::new(*position).into_value_with(ruby)
        }
        AxisEndPoint::ByCoordinate(coord) => {
            YClipPathPathAxisEndPointByCoordinate::new(*coord).into_value_with(ruby)
        }
    }
}

fn make_path_control_point(point: &ClipPathPathControlPoint, ruby: &Ruby) -> Value {
    match point {
        ControlPoint::Absolute(position) => {
            YClipPathPathControlPointAbsolute::new(*position).into_value_with(ruby)
        }
        ControlPoint::Relative(relative) => {
            YClipPathPathControlPointRelative::new(*relative).into_value_with(ruby)
        }
    }
}

fn make_path_arc_radii(radii: &ClipPathPathArcRadii, ruby: &Ruby) -> Value {
    YClipPathPathArcRadii::new(*radii).into_value_with(ruby)
}

#[magnus::wrap(class = "Yass::Declarations::ClipPath::PathCommandEndPointToPosition")]
pub struct YClipPathPathCommandEndPointToPosition {
    position: ClipPathSvgPosition,
}

impl YClipPathPathCommandEndPointToPosition {
    pub fn new(position: ClipPathSvgPosition) -> Self {
        Self { position }
    }

    pub fn x(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> f32 {
        rb_self.position.horizontal
    }

    pub fn y(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> f32 {
        rb_self.position.vertical
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::ClipPath::PathCommandEndPointByCoordinate", mark)]
pub struct YClipPathPathCommandEndPointByCoordinate {
    coord: CachedValue<CoordinatePair<style::values::CSSFloat>>,
}

impl YClipPathPathCommandEndPointByCoordinate {
    pub fn new(coord: CoordinatePair<style::values::CSSFloat>) -> Self {
        Self {
            coord: CachedValue::new(coord, make_path_coordinate_pair),
        }
    }

    pub fn coord(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.coord.get(ruby)
    }
}

impl DataTypeFunctions for YClipPathPathCommandEndPointByCoordinate {
    fn mark(&self, marker: &gc::Marker) {
        self.coord.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::Declarations::ClipPath::PathCoordinatePair")]
pub struct YClipPathPathCoordinatePair {
    coord: CoordinatePair<style::values::CSSFloat>,
}

impl YClipPathPathCoordinatePair {
    pub fn new(coord: CoordinatePair<style::values::CSSFloat>) -> Self {
        Self { coord }
    }

    pub fn x(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> f32 {
        rb_self.coord.x
    }

    pub fn y(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> f32 {
        rb_self.coord.y
    }
}

#[magnus::wrap(class = "Yass::Declarations::ClipPath::PathAxisEndPointToPosition")]
pub struct YClipPathPathAxisEndPointToPosition {
    position: AxisPosition<style::values::CSSFloat>,
}

impl YClipPathPathAxisEndPointToPosition {
    pub fn new(position: AxisPosition<style::values::CSSFloat>) -> Self {
        Self { position }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        match rb_self.position {
            AxisPosition::LengthPercent(value) => YNumber::new(value).into_value_with(ruby),
            AxisPosition::Keyword(keyword) => axis_position_keyword_to_id(keyword, ruby).into_value_with(ruby),
        }
    }
}

#[magnus::wrap(class = "Yass::Declarations::ClipPath::PathAxisEndPointByCoordinate")]
pub struct YClipPathPathAxisEndPointByCoordinate {
    coord: style::values::CSSFloat,
}

impl YClipPathPathAxisEndPointByCoordinate {
    pub fn new(coord: style::values::CSSFloat) -> Self {
        Self { coord }
    }

    pub fn value(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> f32 {
        rb_self.coord
    }
}

#[magnus::wrap(class = "Yass::Declarations::ClipPath::PathControlPointAbsolute")]
pub struct YClipPathPathControlPointAbsolute {
    position: ClipPathSvgPosition,
}

impl YClipPathPathControlPointAbsolute {
    pub fn new(position: ClipPathSvgPosition) -> Self {
        Self { position }
    }

    pub fn x(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> f32 {
        rb_self.position.horizontal
    }

    pub fn y(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> f32 {
        rb_self.position.vertical
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::ClipPath::PathControlPointRelative", mark)]
pub struct YClipPathPathControlPointRelative {
    relative: CachedValue<RelativeControlPoint<style::values::CSSFloat>>,
}

impl YClipPathPathControlPointRelative {
    pub fn new(relative: RelativeControlPoint<style::values::CSSFloat>) -> Self {
        Self {
            relative: CachedValue::new(relative, |relative, ruby| {
                YClipPathPathRelativeControlPoint::new(*relative).into_value_with(ruby)
            }),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.relative.get(ruby)
    }
}

impl DataTypeFunctions for YClipPathPathControlPointRelative {
    fn mark(&self, marker: &gc::Marker) {
        self.relative.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::ClipPath::PathRelativeControlPoint", mark)]
pub struct YClipPathPathRelativeControlPoint {
    coord: CachedValue<CoordinatePair<style::values::CSSFloat>>,
    reference: ControlReference,
}

impl YClipPathPathRelativeControlPoint {
    pub fn new(relative: RelativeControlPoint<style::values::CSSFloat>) -> Self {
        Self {
            coord: CachedValue::new(relative.coord, make_path_coordinate_pair),
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

impl DataTypeFunctions for YClipPathPathRelativeControlPoint {
    fn mark(&self, marker: &gc::Marker) {
        self.coord.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::ClipPath::PathArcRadii", mark)]
pub struct YClipPathPathArcRadii {
    rx: CachedValue<style::values::CSSFloat>,
    ry: CachedValue<style::values::generics::Optional<style::values::CSSFloat>>,
}

impl YClipPathPathArcRadii {
    pub fn new(radii: ClipPathPathArcRadii) -> Self {
        Self {
            rx: CachedValue::new(radii.rx, |rx, ruby| YNumber::new(*rx).into_value_with(ruby)),
            ry: CachedValue::new(radii.ry, |ry, ruby| {
                match ry {
                    style::values::generics::Optional::Some(value) => {
                        YNumber::new(*value).into_value_with(ruby)
                    },

                    style::values::generics::Optional::None => ruby.qnil().into_value_with(ruby),
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

impl DataTypeFunctions for YClipPathPathArcRadii {
    fn mark(&self, marker: &gc::Marker) {
        self.rx.mark(marker);
        self.ry.mark(marker);
    }
}
