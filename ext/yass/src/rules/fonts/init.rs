use magnus::{Error, Module, RModule, Ruby, method};

use crate::rules::fonts::{
    YFontFamilyName,
    YUnicodeRange,
    metrics::YFontMetricsOverride,
    source::{
        YFontFaceSourceFormatKeyword,
        YFontFaceSourceFormatString,
        YFontSourceLocal,
        YFontSourceUrl,
    },
    stretch::{YFontStretch, YFontStretchKeyword, YFontStretchRange},
    style::YFontStyleOblique,
    weight::YFontWeightRange,
};

pub fn init(ruby: &Ruby, yass_module: &RModule) -> Result<(), Error> {
    let unicode_range_class = yass_module.define_class("UnicodeRange", ruby.class_object())?;
    unicode_range_class.define_method("start", method!(YUnicodeRange::start, 0))?;
    unicode_range_class.define_method("end", method!(YUnicodeRange::end, 0))?;

    let font_module = yass_module.define_module("Font")?;

    let font_family_name_class = font_module.define_class("FamilyName", ruby.class_object())?;
    font_family_name_class.define_method("name", method!(YFontFamilyName::name, 0))?;
    font_family_name_class.define_method("syntax", method!(YFontFamilyName::syntax, 0))?;

    let font_metrics_module = font_module.define_module("Metrics")?;

    let font_metrics_override_class = font_metrics_module.define_class("Override", ruby.class_object())?;
    font_metrics_override_class.define_method("value", method!(YFontMetricsOverride::value, 0))?;

    let _font_metrics_override_normal_class = font_metrics_module.define_class("OverrideNormal", ruby.class_object())?;

    let font_source_format_module = font_module.define_module("SourceFormat")?;

    let font_source_format_keyword_class = font_source_format_module.define_class("Keyword", ruby.class_object())?;
    font_source_format_keyword_class.define_method("value", method!(YFontFaceSourceFormatKeyword::value, 0))?;

    let font_source_format_string_class = font_source_format_module.define_class("String", ruby.class_object())?;
    font_source_format_string_class.define_method("value", method!(YFontFaceSourceFormatString::value, 0))?;

    let font_source_module = font_module.define_module("Source")?;

    let font_source_url_class = font_source_module.define_class("Url", ruby.class_object())?;
    font_source_url_class.define_method("specified_url", method!(YFontSourceUrl::specified_url, 0))?;
    font_source_url_class.define_method("format_hint", method!(YFontSourceUrl::format_hint, 0))?;

    let font_source_local_class = font_source_module.define_class("Local", ruby.class_object())?;
    font_source_local_class.define_method("family_name", method!(YFontSourceLocal::family_name, 0))?;

    let font_stretch_module = yass_module.define_module("FontStretch")?;

    let font_stretch_range_class = font_stretch_module.define_class("Range", ruby.class_object())?;
    font_stretch_range_class.define_method("begin", method!(YFontStretchRange::begin, 0))?;
    font_stretch_range_class.define_method("end", method!(YFontStretchRange::end, 0))?;

    let font_stretch_stretch_class = font_stretch_module.define_class("Stretch", ruby.class_object())?;
    font_stretch_stretch_class.define_method("value", method!(YFontStretch::value, 0))?;

    let font_stretch_keyword_class = font_stretch_module.define_class("Keyword", ruby.class_object())?;
    font_stretch_keyword_class.define_method("value", method!(YFontStretchKeyword::value, 0))?;

    let font_style_module = yass_module.define_module("FontStyle")?;

    let _font_style_italic_class = font_style_module.define_class("Italic", ruby.class_object())?;

    let font_style_oblique_class = font_style_module.define_class("Oblique", ruby.class_object())?;
    font_style_oblique_class.define_method("angle1", method!(YFontStyleOblique::angle1, 0))?;
    font_style_oblique_class.define_method("angle2", method!(YFontStyleOblique::angle2, 0))?;

    let font_weight_module = yass_module.define_module("FontWeight")?;

    let _font_weight_normal_class = font_weight_module.define_class("Normal", ruby.class_object())?;

    let font_weight_range_class = font_weight_module.define_class("Range", ruby.class_object())?;
    font_weight_range_class.define_method("begin", method!(YFontWeightRange::begin, 0))?;
    font_weight_range_class.define_method("end", method!(YFontWeightRange::end, 0))?;

    let _system_font_class = yass_module.define_class("SystemFont", ruby.class_object())?;

    Ok(())
}
