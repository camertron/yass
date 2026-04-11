use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc, typed_data, value::Id};
use style::values::{generics::transform::TransformOrigin, specified::{Length, position::{HorizontalPositionKeyword, VerticalPositionKeyword}, transform::OriginComponent}};

use crate::{cached_value::CachedValue, declarations::{images::{horizontal_keyword_to_id, vertical_keyword_to_id}, length::length_to_value, size::YLengthPercentage}};

fn make_horizontal_origin_component(component: &OriginComponent<HorizontalPositionKeyword>, ruby: &Ruby) -> Value {
    match component {
        OriginComponent::Center => YOriginCenter::new().into_value_with(ruby),
        OriginComponent::Length(lp) => YLengthPercentage::new(lp.clone()).into_value_with(ruby),
        OriginComponent::Side(keyword) => YSideHorizontalOriginComponent::new(*keyword).into_value_with(ruby),
    }
}

fn make_vertical_origin_component(component: &OriginComponent<VerticalPositionKeyword>, ruby: &Ruby) -> Value {
    match component {
        OriginComponent::Center => YOriginCenter::new().into_value_with(ruby),
        OriginComponent::Length(lp) => YLengthPercentage::new(lp.clone()).into_value_with(ruby),
        OriginComponent::Side(keyword) => YSideVerticalOriginComponent::new(*keyword).into_value_with(ruby),
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::TransformOrigin", mark)]
pub struct YTransformOrigin {
    horizontal: CachedValue<OriginComponent<HorizontalPositionKeyword>>,
    vertical: CachedValue<OriginComponent<VerticalPositionKeyword>>,
    depth: CachedValue<Length>,
}

impl YTransformOrigin {
    pub fn new(specified_value: Box<TransformOrigin<OriginComponent<HorizontalPositionKeyword>, OriginComponent<VerticalPositionKeyword>, Length>>) -> Self {
        let to = *specified_value;

        Self {
            horizontal: CachedValue::new(to.horizontal, make_horizontal_origin_component),
            vertical: CachedValue::new(to.vertical, make_vertical_origin_component),
            depth: CachedValue::new(to.depth, |length, ruby| length_to_value(length, ruby)),
        }
    }

    pub fn horizontal(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.horizontal.get(ruby)
    }

    pub fn vertical(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.vertical.get(ruby)
    }

    pub fn depth(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.depth.get(ruby)
    }
}

impl DataTypeFunctions for YTransformOrigin {
    fn mark(&self, marker: &gc::Marker) {
        self.horizontal.mark(marker);
        self.vertical.mark(marker);
        self.depth.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::Declarations::TransformOrigin::Center")]
pub struct YOriginCenter {}

impl YOriginCenter {
    pub fn new() -> Self {
        Self {}
    }
}

#[magnus::wrap(class = "Yass::Declarations::TransformOrigin::SideHorizontalComponent")]
pub struct YSideHorizontalOriginComponent {
    keyword: HorizontalPositionKeyword,
}

impl YSideHorizontalOriginComponent {
    pub fn new(keyword: HorizontalPositionKeyword) -> Self {
        Self { keyword }
    }

    pub fn side(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        horizontal_keyword_to_id(rb_self.keyword, ruby)
    }
}

#[magnus::wrap(class = "Yass::Declarations::TransformOrigin::SideVerticalComponent")]
pub struct YSideVerticalOriginComponent {
    keyword: VerticalPositionKeyword,
}

impl YSideVerticalOriginComponent {
    pub fn new(keyword: VerticalPositionKeyword) -> Self {
        Self { keyword }
    }

    pub fn side(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        vertical_keyword_to_id(rb_self.keyword, ruby)
    }
}
