use magnus::{DataTypeFunctions, Error, IntoValue, RArray, Ruby, TypedData, Value, gc, typed_data};
use style::{
    properties::longhands::filter::SpecifiedValue,
    values::specified::effects::{Filter as SpecifiedFilter, FilterFactor},
};
use style_traits::ToCss;

use crate::{
    cached_value_list::CachedValueList, declarations::{filter::{blur::YFilterBlur, brightness::YFilterBrightness, contrast::YFilterContrast, drop_shadow::YFilterDropShadow, grayscale::YFilterGrayscale, hue_rotate::YFilterHueRotate, invert::YFilterInvert, opacity::YFilterOpacity, saturate::YFilterSaturate, sepia::YFilterSepia}, number::YNumber, percentage::YPercentage},
};

pub mod blur;
pub mod brightness;
pub mod contrast;
pub mod drop_shadow;
pub mod grayscale;
pub mod hue_rotate;
pub mod init;
pub mod invert;
pub mod opacity;
pub mod saturate;
pub mod sepia;

fn filter_factor_css_to_value(css: &str, ruby: &Ruby) -> Value {
    if let Some(percentage) = css.strip_suffix('%').and_then(|raw| raw.parse::<f32>().ok()) {
        return YPercentage::new(percentage / 100.0).into_value_with(ruby);
    }

    if let Ok(number) = css.parse::<f32>() {
        return YNumber::new(number).into_value_with(ruby);
    }

    ruby.str_new(css).into_value_with(ruby)
}

pub fn filter_factor_to_value(factor: &FilterFactor, ruby: &Ruby) -> Value {
    filter_factor_css_to_value(&factor.to_css_string(), ruby)
}

pub fn filter_to_value(value: &SpecifiedFilter, ruby: &Ruby) -> Value {
    match value {
        SpecifiedFilter::Blur(length) => YFilterBlur::new(length.clone()).into_value_with(ruby),
        SpecifiedFilter::Brightness(factor) => YFilterBrightness::new(factor.clone()).into_value_with(ruby),
        SpecifiedFilter::Contrast(factor) => YFilterContrast::new(factor.clone()).into_value_with(ruby),
        SpecifiedFilter::Grayscale(factor) => YFilterGrayscale::new(factor.clone()).into_value_with(ruby),
        SpecifiedFilter::HueRotate(angle) => YFilterHueRotate::new(angle.clone()).into_value_with(ruby),
        SpecifiedFilter::Invert(factor) => YFilterInvert::new(factor.clone()).into_value_with(ruby),
        SpecifiedFilter::Opacity(factor) => YFilterOpacity::new(factor.clone()).into_value_with(ruby),
        SpecifiedFilter::Saturate(factor) => YFilterSaturate::new(factor.clone()).into_value_with(ruby),
        SpecifiedFilter::Sepia(factor) => YFilterSepia::new(factor.clone()).into_value_with(ruby),
        SpecifiedFilter::DropShadow(shadow) => YFilterDropShadow::new(shadow.clone()).into_value_with(ruby),
        SpecifiedFilter::Url(_) => unreachable!("servo specified filter URLs are impossible"),
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Filter", mark)]
pub struct YFilter {
    values: CachedValueList<SpecifiedFilter>,
}

impl YFilter {
    pub fn new(specified_value: SpecifiedValue) -> Self {
        Self {
            values: CachedValueList::new(specified_value.0.to_vec(), |value, _ctx, ruby| {
                filter_to_value(value, ruby)
            }),
        }
    }

    pub fn values(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        rb_self.values.to_a(ruby)
    }
}

impl DataTypeFunctions for YFilter {
    fn mark(&self, marker: &gc::Marker) {
        self.values.mark(marker);
    }
}
