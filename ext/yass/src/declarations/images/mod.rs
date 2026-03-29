use magnus::{IntoValue, Ruby, Value, value::Id};
use style::{
    values::{
        generics::image::{GradientCompatMode, ShapeExtent},
        specified::{
            image::{Gradient, Image as SpecifiedImage},
            position::{HorizontalPositionKeyword, VerticalPositionKeyword},
        },
    },
};
use style_traits::ToCss;

use crate::declarations::{images::{conic_gradient::YConicGradient, cross_fade::YCrossFade, image_set::YImageSet, light_dark::YImageLightDark, linear_gradient::YLinearGradient, none::YImageNone, radial_gradient::YImageRadialGradient, url::YImageUrl}};

pub mod angle_or_percentage;
pub mod circle;
pub mod conic_gradient;
pub mod cross_fade;
pub mod cross_fade_element;
pub mod cross_fade_image;
pub mod ellipse;
pub mod ending_shape;
pub mod gradient_angle_or_percentage_item;
pub mod gradient_length_percentage_item;
pub mod horizontal_position_component;
pub mod init;
pub mod image_set;
pub mod image_set_item;
pub mod light_dark;
pub mod line_direction;
pub mod linear_gradient;
pub mod none;
pub mod position;
pub mod radial_gradient;
pub mod url;
pub mod vertical_position_component;

pub fn image_to_value(image: &SpecifiedImage, ruby: &Ruby) -> Value {
    match image {
        SpecifiedImage::None => YImageNone::new().into_value_with(ruby),
        SpecifiedImage::Url(url) => YImageUrl::new(url.clone()).into_value_with(ruby),
        SpecifiedImage::Gradient(gradient) => gradient_to_value(gradient, ruby),
        SpecifiedImage::CrossFade(cross_fade) => {
            YCrossFade::new((**cross_fade).clone()).into_value_with(ruby)
        },
        SpecifiedImage::ImageSet(image_set) => {
            YImageSet::new((**image_set).clone()).into_value_with(ruby)
        },
        SpecifiedImage::LightDark(light_dark) => {
            YImageLightDark::new((**light_dark).clone()).into_value_with(ruby)
        },
        _ => ruby.str_new(&image.to_css_string()).into_value_with(ruby),
    }
}

fn gradient_to_value(gradient: &Gradient, ruby: &Ruby) -> Value {
    match gradient {
        Gradient::Linear { .. } => YLinearGradient::new(gradient.clone()).into_value_with(ruby),
        Gradient::Radial { .. } => YImageRadialGradient::new(gradient.clone()).into_value_with(ruby),
        Gradient::Conic { .. } => YConicGradient::new(gradient.clone()).into_value_with(ruby),
    }
}

pub fn horizontal_keyword_to_id(keyword: HorizontalPositionKeyword, ruby: &Ruby) -> Id {
    match keyword {
        HorizontalPositionKeyword::Left => ruby.intern("left"),
        HorizontalPositionKeyword::Right => ruby.intern("right"),
    }
}

pub fn vertical_keyword_to_id(keyword: VerticalPositionKeyword, ruby: &Ruby) -> Id {
    match keyword {
        VerticalPositionKeyword::Top => ruby.intern("top"),
        VerticalPositionKeyword::Bottom => ruby.intern("bottom"),
    }
}

pub fn shape_extent_to_id(extent: ShapeExtent, ruby: &Ruby) -> Id {
    match extent {
        ShapeExtent::ClosestSide => ruby.intern("closest_side"),
        ShapeExtent::FarthestSide => ruby.intern("farthest_side"),
        ShapeExtent::ClosestCorner => ruby.intern("closest_corner"),
        ShapeExtent::FarthestCorner => ruby.intern("farthest_corner"),
        ShapeExtent::Contain => ruby.intern("contain"),
        ShapeExtent::Cover => ruby.intern("cover"),
    }
}

pub fn compat_mode_to_id(mode: GradientCompatMode, ruby: &Ruby) -> Id {
    match mode {
        GradientCompatMode::Modern => ruby.intern("modern"),
        GradientCompatMode::WebKit => ruby.intern("webkit"),
        GradientCompatMode::Moz => ruby.intern("moz"),
    }
}
