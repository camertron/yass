use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc, typed_data, value::Id};
use style::values::{
    generics::{
        motion::RaySize,
        position::GenericPositionOrAuto,
    },
    specified::{
        Angle,
        PositionComponent,
        motion::{CoordBox, OffsetPath, OffsetPathFunction, RayFunction},
        position::{HorizontalPositionKeyword, VerticalPositionKeyword},
    },
};
use style_traits::ToCss;

use crate::{
    cached_value::CachedValue,
    declarations::{
        angle::YAngle,
        clip_path::{make_basic_shape, ClipPathBasicShape},
        horizontal_position_component::make_horizontal_position_component,
        vertical_position_component::make_vertical_position_component,
    },
};

type OffsetPathPositionOrAuto = GenericPositionOrAuto<
    style::values::generics::position::GenericPosition<
        PositionComponent<HorizontalPositionKeyword>,
        PositionComponent<VerticalPositionKeyword>,
    >
>;

fn coord_box_to_id(coord_box: CoordBox, ruby: &Ruby) -> Id {
    match coord_box {
        CoordBox::ContentBox => ruby.intern("content_box"),
        CoordBox::PaddingBox => ruby.intern("padding_box"),
        CoordBox::BorderBox => ruby.intern("border_box"),
        CoordBox::FillBox => ruby.intern("fill_box"),
        CoordBox::StrokeBox => ruby.intern("stroke_box"),
        CoordBox::ViewBox => ruby.intern("view_box"),
    }
}

fn ray_size_to_id(ray_size: RaySize, ruby: &Ruby) -> Id {
    match ray_size {
        RaySize::ClosestSide => ruby.intern("closest_side"),
        RaySize::ClosestCorner => ruby.intern("closest_corner"),
        RaySize::FarthestSide => ruby.intern("farthest_side"),
        RaySize::FarthestCorner => ruby.intern("farthest_corner"),
        RaySize::Sides => ruby.intern("sides"),
    }
}

fn make_offset_path_position_or_auto(position: &OffsetPathPositionOrAuto, ruby: &Ruby) -> Value {
    match position {
        GenericPositionOrAuto::Auto => YOffsetPathPositionAuto::new().into_value_with(ruby),
        GenericPositionOrAuto::Position(pos) => {
            YOffsetPathPosition::new(pos.clone()).into_value_with(ruby)
        }
    }
}

fn make_offset_path_function(path: &OffsetPathFunction, ruby: &Ruby) -> Value {
    match path {
        OffsetPathFunction::Ray(ray) => {
            YOffsetPathRay::new(ray.clone()).into_value_with(ruby)
        }
        OffsetPathFunction::Url(url) => {
            YOffsetPathUrl::new(url.to_css_string()).into_value_with(ruby)
        }
        OffsetPathFunction::Shape(basic_shape) => {
            make_basic_shape(basic_shape as &ClipPathBasicShape, ruby)
        }
    }
}

fn make_offset_path(offset_path: &OffsetPath, ruby: &Ruby) -> Value {
    match offset_path {
        OffsetPath::None => YOffsetPathNone::new().into_value_with(ruby),
        OffsetPath::CoordBox(coord_box) => {
            YOffsetPathCoordBox::new(*coord_box).into_value_with(ruby)
        }
        OffsetPath::OffsetPath { path, coord_box } => {
            YOffsetPathFunction::new((**path).clone(), *coord_box).into_value_with(ruby)
        }
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::OffsetPath", mark)]
pub struct YOffsetPath {
    value: CachedValue<OffsetPath>,
}

impl YOffsetPath {
    pub fn new(offset_path: OffsetPath) -> Self {
        Self {
            value: CachedValue::new(offset_path, make_offset_path),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.value.get(ruby)
    }
}

impl DataTypeFunctions for YOffsetPath {
    fn mark(&self, marker: &gc::Marker) {
        self.value.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::Declarations::OffsetPath::None")]
pub struct YOffsetPathNone {}

impl YOffsetPathNone {
    pub fn new() -> Self {
        Self {}
    }
}

#[magnus::wrap(class = "Yass::Declarations::OffsetPath::CoordBox")]
pub struct YOffsetPathCoordBox {
    coord_box: CoordBox,
}

impl YOffsetPathCoordBox {
    pub fn new(coord_box: CoordBox) -> Self {
        Self { coord_box }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        coord_box_to_id(rb_self.coord_box, ruby)
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::OffsetPath::Function", mark)]
pub struct YOffsetPathFunction {
    path: CachedValue<OffsetPathFunction>,
    coord_box: CoordBox,
}

impl YOffsetPathFunction {
    pub fn new(path: OffsetPathFunction, coord_box: CoordBox) -> Self {
        Self {
            path: CachedValue::new(path, make_offset_path_function),
            coord_box,
        }
    }

    pub fn path(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.path.get(ruby)
    }

    pub fn coord_box(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        coord_box_to_id(rb_self.coord_box, ruby)
    }
}

impl DataTypeFunctions for YOffsetPathFunction {
    fn mark(&self, marker: &gc::Marker) {
        self.path.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::OffsetPath::Ray", mark)]
pub struct YOffsetPathRay {
    angle: CachedValue<Angle>,
    size: RaySize,
    contain: bool,
    position: CachedValue<OffsetPathPositionOrAuto>,
}

impl YOffsetPathRay {
    pub fn new(ray: RayFunction) -> Self {
        Self {
            angle: CachedValue::new(ray.angle, |angle, ruby| {
                YAngle::new(*angle).into_value_with(ruby)
            }),

            size: ray.size,
            contain: ray.contain,
            position: CachedValue::new(ray.position, make_offset_path_position_or_auto),
        }
    }

    pub fn angle(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.angle.get(ruby)
    }

    pub fn size(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        ray_size_to_id(rb_self.size, ruby)
    }

    pub fn contain(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.contain
    }

    pub fn position(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.position.get(ruby)
    }
}

impl DataTypeFunctions for YOffsetPathRay {
    fn mark(&self, marker: &gc::Marker) {
        self.angle.mark(marker);
        self.position.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::Declarations::OffsetPath::Url")]
pub struct YOffsetPathUrl {
    css: String,
}

impl YOffsetPathUrl {
    pub fn new(css: String) -> Self {
        Self { css }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        ruby.str_new(&rb_self.css).into_value_with(ruby)
    }
}

#[magnus::wrap(class = "Yass::Declarations::OffsetPath::PositionAuto")]
pub struct YOffsetPathPositionAuto {}

impl YOffsetPathPositionAuto {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::OffsetPath::Position", mark)]
pub struct YOffsetPathPosition {
    horizontal: CachedValue<PositionComponent<HorizontalPositionKeyword>>,
    vertical: CachedValue<PositionComponent<VerticalPositionKeyword>>,
}

impl YOffsetPathPosition {
    pub fn new(
        position: style::values::generics::position::GenericPosition<
            PositionComponent<HorizontalPositionKeyword>,
            PositionComponent<VerticalPositionKeyword>,
        >,
    ) -> Self {
        Self {
            horizontal: CachedValue::new(position.horizontal, |component, ruby| {
                make_horizontal_position_component(component, ruby)
            }),
            vertical: CachedValue::new(position.vertical, |component, ruby| {
                make_vertical_position_component(component, ruby)
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

impl DataTypeFunctions for YOffsetPathPosition {
    fn mark(&self, marker: &gc::Marker) {
        self.horizontal.mark(marker);
        self.vertical.mark(marker);
    }
}
