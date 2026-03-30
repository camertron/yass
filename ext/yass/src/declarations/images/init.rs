use magnus::{Error, Module, RModule, Ruby, method};

use crate::declarations::{horizontal_position_component::{YLengthHorizontalPositionComponent, YSideHorizontalPositionComponent}, vertical_position_component::{YLengthVerticalPositionComponent, YSideVerticalPositionComponent}};
use crate::declarations::images::{
    angle_or_percentage::YAngleOrPercentage, circle::{YExtentCircle, YRadiusCircle}, conic_gradient::YConicGradient, cross_fade::YCrossFade, cross_fade_element::YCrossFadeElement, cross_fade_image::{YCrossFadeColor, YCrossFadeImage}, ellipse::{YExtentEllipse, YRadiiEllipse}, ending_shape::YEndingShape, gradient_angle_or_percentage_item::{YComplexColorStopAngle, YInterpolationHintAngle, YSimpleColorStopAngle}, gradient_length_percentage_item::{YComplexColorStopLength, YInterpolationHintLength, YSimpleColorStopLength}, image_set::YImageSet, image_set_item::YImageSetItem, light_dark::YImageLightDark, line_direction::{YAngleLineDirection, YCornerLineDirection, YHorizontalLineDirection, YVerticalLineDirection}, linear_gradient::YLinearGradient, position::YImagePosition, radial_gradient::YImageRadialGradient, url::YImageUrl
};

pub fn init(ruby: &Ruby, declarations_module: &RModule) -> Result<(), Error> {
    let image_module = declarations_module.define_module("Image")?;

    let radius_circle_class = image_module.define_class("RadiusCircle", ruby.class_object())?;
    radius_circle_class.define_method("length", method!(YRadiusCircle::length, 0))?;

    let extent_circle_class = image_module.define_class("ExtentCircle", ruby.class_object())?;
    extent_circle_class.define_method("extent", method!(YExtentCircle::extent, 0))?;

    let _none_class = image_module.define_class("None", ruby.class_object())?;

    let url_class = image_module.define_class("Url", ruby.class_object())?;
    url_class.define_method("resolved", method!(YImageUrl::resolved, 0))?;
    url_class.define_method("original", method!(YImageUrl::original, 0))?;
    url_class.define_method("invalid?", method!(YImageUrl::invalid, 0))?;

    let light_dark_class = image_module.define_class("LightDark", ruby.class_object())?;
    light_dark_class.define_method("light", method!(YImageLightDark::light, 0))?;
    light_dark_class.define_method("dark", method!(YImageLightDark::dark, 0))?;

    let cross_fade_class = image_module.define_class("CrossFade", ruby.class_object())?;
    cross_fade_class.define_method("elements", method!(YCrossFade::elements, 0))?;

    let cross_fade_image_class = image_module.define_class("CrossFadeImage", ruby.class_object())?;
    cross_fade_image_class.define_method("image", method!(YCrossFadeImage::image, 0))?;

    let cross_fade_color_class = image_module.define_class("CrossFadeColor", ruby.class_object())?;
    cross_fade_color_class.define_method("image", method!(YCrossFadeColor::color, 0))?;

    let cross_fade_element_class = image_module.define_class("CrossFadeElement", ruby.class_object())?;
    cross_fade_element_class.define_method("percent", method!(YCrossFadeElement::percent, 0))?;
    cross_fade_element_class.define_method("image", method!(YCrossFadeElement::image, 0))?;

    let image_set_class = image_module.define_class("ImageSet", ruby.class_object())?;
    image_set_class.define_method("items", method!(YImageSet::items, 0))?;

    let image_set_item_class = image_module.define_class("ImageSetItem", ruby.class_object())?;
    image_set_item_class.define_method("image", method!(YImageSetItem::image, 0))?;
    image_set_item_class.define_method("resolution", method!(YImageSetItem::resolution, 0))?;
    image_set_item_class.define_method("has_mime_type?", method!(YImageSetItem::has_mime_type, 0))?;
    image_set_item_class.define_method("mime_type", method!(YImageSetItem::mime_type, 0))?;

    let angle_line_direction_class = image_module.define_class("AngleLineDirection", ruby.class_object())?;
    angle_line_direction_class.define_method("angle", method!(YAngleLineDirection::angle, 0))?;

    let horizontal_line_direction_class = image_module.define_class("HorizontalLineDirection", ruby.class_object())?;
    horizontal_line_direction_class.define_method("direction", method!(YHorizontalLineDirection::direction, 0))?;

    let vertical_line_direction_class = image_module.define_class("VerticalLineDirection", ruby.class_object())?;
    vertical_line_direction_class.define_method("direction", method!(YVerticalLineDirection::direction, 0))?;

    let corner_line_direction_class = image_module.define_class("CornerLineDirection", ruby.class_object())?;
    corner_line_direction_class.define_method("horizontal_direction", method!(YCornerLineDirection::horizontal_direction, 0))?;
    corner_line_direction_class.define_method("vertical_direction", method!(YCornerLineDirection::vertical_direction, 0))?;

    let position_class = image_module.define_class("Position", ruby.class_object())?;
    position_class.define_method("horizontal_position", method!(YImagePosition::horizontal_position, 0))?;
    position_class.define_method("vertical_position", method!(YImagePosition::vertical_position, 0))?;

    let _center_horizontal_position_component_class = image_module.define_class("CenterHorizontalPositionComponent", ruby.class_object())?;

    let length_horizontal_position_component_class = image_module.define_class("LengthHorizontalPositionComponent", ruby.class_object())?;
    length_horizontal_position_component_class.define_method("length", method!(YLengthHorizontalPositionComponent::length, 0))?;

    let side_horizontal_position_component_class = image_module.define_class("SideHorizontalPositionComponent", ruby.class_object())?;
    side_horizontal_position_component_class.define_method("side", method!(YSideHorizontalPositionComponent::side, 0))?;
    side_horizontal_position_component_class.define_method("offset", method!(YSideHorizontalPositionComponent::offset, 0))?;

    let _center_vertical_position_component_class = image_module.define_class("CenterVerticalPositionComponent", ruby.class_object())?;

    let length_vertical_position_component_class = image_module.define_class("LengthVerticalPositionComponent", ruby.class_object())?;
    length_vertical_position_component_class.define_method("length", method!(YLengthVerticalPositionComponent::length, 0))?;

    let side_vertical_position_component_class = image_module.define_class("SideVerticalPositionComponent", ruby.class_object())?;
    side_vertical_position_component_class.define_method("position", method!(YSideVerticalPositionComponent::position, 0))?;
    side_vertical_position_component_class.define_method("offset", method!(YSideVerticalPositionComponent::offset, 0))?;

    let ending_shape_class = image_module.define_class("EndingShape", ruby.class_object())?;
    ending_shape_class.define_method("value", method!(YEndingShape::value, 0))?;

    let radii_ellipse_class = image_module.define_class("RadiiEllipse", ruby.class_object())?;
    radii_ellipse_class.define_method("x", method!(YRadiiEllipse::x, 0))?;
    radii_ellipse_class.define_method("y", method!(YRadiiEllipse::y, 0))?;

    let extent_ellipse_class = image_module.define_class("ExtentEllipse", ruby.class_object())?;
    extent_ellipse_class.define_method("extent", method!(YExtentEllipse::extent, 0))?;

    let angle_or_percentage_class = image_module.define_class("AngleOrPercentage", ruby.class_object())?;
    angle_or_percentage_class.define_method("value", method!(YAngleOrPercentage::value, 0))?;

    let gradient_module = image_module.define_module("Gradient")?;

    let simple_color_stop_angle_class = gradient_module.define_class("SimpleColorStopAngle", ruby.class_object())?;
    simple_color_stop_angle_class.define_method("kind", method!(YSimpleColorStopAngle::kind, 0))?;
    simple_color_stop_angle_class.define_method("color", method!(YSimpleColorStopAngle::color, 0))?;
    simple_color_stop_angle_class.define_method("position", method!(YSimpleColorStopAngle::position, 0))?;

    let complex_color_stop_angle_class = gradient_module.define_class("ComplexColorStopAngle", ruby.class_object())?;
    complex_color_stop_angle_class.define_method("kind", method!(YComplexColorStopAngle::kind, 0))?;
    complex_color_stop_angle_class.define_method("color", method!(YComplexColorStopAngle::color, 0))?;
    complex_color_stop_angle_class.define_method("position", method!(YComplexColorStopAngle::position, 0))?;

    let interpolation_hint_angle_class = gradient_module.define_class("InterpolationHintAngle", ruby.class_object())?;
    interpolation_hint_angle_class.define_method("kind", method!(YInterpolationHintAngle::kind, 0))?;
    interpolation_hint_angle_class.define_method("color", method!(YInterpolationHintAngle::color, 0))?;
    interpolation_hint_angle_class.define_method("position", method!(YInterpolationHintAngle::position, 0))?;

    let simple_color_stop_length_class = gradient_module.define_class("SimpleColorStopLength", ruby.class_object())?;
    simple_color_stop_length_class.define_method("color", method!(YSimpleColorStopLength::color, 0))?;

    let complex_color_stop_length_class = gradient_module.define_class("ComplexColorStopLength", ruby.class_object())?;
    complex_color_stop_length_class.define_method("color", method!(YComplexColorStopLength::color, 0))?;
    complex_color_stop_length_class.define_method("position", method!(YComplexColorStopLength::position, 0))?;

    let interpolation_hint_length_class = gradient_module.define_class("InterpolationHintLength", ruby.class_object())?;
    interpolation_hint_length_class.define_method("position", method!(YInterpolationHintLength::position, 0))?;

    let linear_gradient_class = image_module.define_class("LinearGradient", ruby.class_object())?;
    linear_gradient_class.define_method("repeating?", method!(YLinearGradient::repeating, 0))?;
    linear_gradient_class.define_method("compat_mode", method!(YLinearGradient::compat_mode, 0))?;
    linear_gradient_class.define_method("direction", method!(YLinearGradient::direction, 0))?;
    linear_gradient_class.define_method("color_interpolation_method", method!(YLinearGradient::color_interpolation_method, 0))?;
    linear_gradient_class.define_method("items", method!(YLinearGradient::items, 0))?;

    let radial_gradient_class = image_module.define_class("RadialGradient", ruby.class_object())?;
    radial_gradient_class.define_method("repeating?", method!(YImageRadialGradient::repeating, 0))?;
    radial_gradient_class.define_method("compat_mode", method!(YImageRadialGradient::compat_mode, 0))?;
    radial_gradient_class.define_method("shape", method!(YImageRadialGradient::shape, 0))?;
    radial_gradient_class.define_method("position", method!(YImageRadialGradient::position, 0))?;
    radial_gradient_class.define_method("color_interpolation_method", method!(YImageRadialGradient::color_interpolation_method, 0))?;
    radial_gradient_class.define_method("items", method!(YImageRadialGradient::items, 0))?;

    let conic_gradient_class = image_module.define_class("ConicGradient", ruby.class_object())?;
    conic_gradient_class.define_method("kind", method!(YConicGradient::kind, 0))?;
    conic_gradient_class.define_method("repeating?", method!(YConicGradient::repeating, 0))?;
    conic_gradient_class.define_method("angle", method!(YConicGradient::angle, 0))?;
    conic_gradient_class.define_method("position", method!(YConicGradient::position, 0))?;
    conic_gradient_class.define_method("color_interpolation_method", method!(YConicGradient::color_interpolation_method, 0))?;
    conic_gradient_class.define_method("items", method!(YConicGradient::items, 0))?;

    Ok(())
}
