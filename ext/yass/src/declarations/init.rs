use magnus::{Error, Module, RModule, Ruby, method};

use crate::declarations::align_content::YAlignContent;
use crate::declarations::align_items::YAlignItems;
use crate::declarations::align_self::YAlignSelf;
use crate::declarations::alignment_baseline::YAlignmentBaseline;
use crate::declarations::angle::YAngle;
use crate::declarations::animation_composition::YAnimationComposition;
use crate::declarations::animation_delay::YAnimationDelay;
use crate::declarations::animation_direction::YAnimationDirection;
use crate::declarations::animation_duration::{YAnimationDuration, YAnimationDurationValue};
use crate::declarations::animation_fill_mode::YAnimationFillMode;
use crate::declarations::animation_iteration_count::YAnimationIterationCount;
use crate::declarations::animation_name::YAnimationName;
use crate::declarations::animation_play_state::YAnimationPlayState;
use crate::declarations::animation_range_end::YAnimationRangeEnd;
use crate::declarations::animation_range_start::YAnimationRangeStart;
use crate::declarations::animation_timeline::YAnimationTimeline;
use crate::declarations::animation_timing_function::YAnimationTimingFunction;
use crate::declarations::animation;
use crate::declarations::aspect_ratio::YAspectRatio;
use crate::declarations::backdrop_filter::YBackdropFilter;
use crate::declarations::backface_visibility::YBackfaceVisibility;
use crate::declarations::background_attachment::YBackgroundAttachment;
use crate::declarations::background_blend_mode::YBackgroundBlendMode;
use crate::declarations::background_clip::YBackgroundClip;
use crate::declarations::background_color::YBackgroundColor;
use crate::declarations::background_image::YBackgroundImage;
use crate::declarations::background_origin::YBackgroundOrigin;
use crate::declarations::background_position_x::YBackgroundPositionX;
use crate::declarations::background_position_y::YBackgroundPositionY;
use crate::declarations::background_repeat::{YBackgroundRepeat, YBackgroundRepeatValue};
use crate::declarations::background_size::{YBackgroundSize, YBackgroundSizeExplicitSize};
use crate::declarations::baseline_shift::{YBaselineShiftKeyword, YBaselineShiftLength};
use crate::declarations::baseline_source::YBaselineSource;
use crate::declarations::block_size::YBlockSize;
use crate::declarations::border_block_end_color::YBorderBlockEndColor;
use crate::declarations::border_block_end_style::YBorderBlockEndStyle;
use crate::declarations::border_block_end_width::YBorderBlockEndWidth;
use crate::declarations::border_block_start_color::YBorderBlockStartColor;
use crate::declarations::border_block_start_style::YBorderBlockStartStyle;
use crate::declarations::border_block_start_width::YBorderBlockStartWidth;
use crate::declarations::border_bottom_color::YBorderBottomColor;
use crate::declarations::border_bottom_left_radius::YBorderBottomLeftRadius;
use crate::declarations::border_bottom_right_radius::YBorderBottomRightRadius;
use crate::declarations::border_bottom_style::YBorderBottomStyle;
use crate::declarations::border_bottom_width::YBorderBottomWidth;
use crate::declarations::border_collapse::YBorderCollapse;
use crate::declarations::border_end_end_radius::YBorderEndEndRadius;
use crate::declarations::border_end_start_radius::YBorderEndStartRadius;
use crate::declarations::border_image_outset::YBorderImageOutset;
use crate::declarations::border_image_repeat::YBorderImageRepeat;
use crate::declarations::border_image_slice::YBorderImageSlice;
use crate::declarations::border_image_source::YBorderImageSource;
use crate::declarations::border_image_width::YBorderImageWidth;
use crate::declarations::border_inline_end_color::YBorderInlineEndColor;
use crate::declarations::border_inline_end_style::YBorderInlineEndStyle;
use crate::declarations::border_inline_end_width::YBorderInlineEndWidth;
use crate::declarations::border_inline_start_color::YBorderInlineStartColor;
use crate::declarations::border_inline_start_style::YBorderInlineStartStyle;
use crate::declarations::border_inline_start_width::YBorderInlineStartWidth;
use crate::declarations::border_left_color::YBorderLeftColor;
use crate::declarations::border_left_style::YBorderLeftStyle;
use crate::declarations::border_left_width::YBorderLeftWidth;
use crate::declarations::border_right_color::YBorderRightColor;
use crate::declarations::border_right_style::YBorderRightStyle;
use crate::declarations::border_right_width::YBorderRightWidth;
use crate::declarations::border_spacing::YBorderSpacing;
use crate::declarations::border_start_end_radius::YBorderStartEndRadius;
use crate::declarations::border_start_start_radius::YBorderStartStartRadius;
use crate::declarations::border_top_color::YBorderTopColor;
use crate::declarations::border_top_left_radius::YBorderTopLeftRadius;
use crate::declarations::border_top_right_radius::YBorderTopRightRadius;
use crate::declarations::border_top_style::YBorderTopStyle;
use crate::declarations::border_top_width::YBorderTopWidth;
use crate::declarations::bottom::YBottom;
use crate::declarations::box_shadow::{YBoxShadow, YBoxShadowShadow};
use crate::declarations::box_sizing::YBoxSizing;
use crate::declarations::calc;
use crate::declarations::caption_side::YCaptionSide;
use crate::declarations::caret_color::YCaretColor;
use crate::declarations::channel_keyword::YChannelKeyword;
use crate::declarations::clear::YClear;
use crate::declarations::clip_path;
use crate::declarations::clip::{YClip, YClipLength, YClipRect};
use crate::declarations::color_scheme::YColorScheme;
use crate::declarations::color::{self, YColor};
use crate::declarations::offset_path::{YOffsetPath, YOffsetPathCoordBox, YOffsetPathFunction, YOffsetPathRay, YOffsetPathUrl, YOffsetPathPositionAuto, YOffsetPathPosition};
use crate::declarations::column_count::YColumnCountInteger;
use crate::declarations::column_gap::{YColumnGap, YColumnGapLengthPercentage};
use crate::declarations::column_span::YColumnSpan;
use crate::declarations::column_width::{YColumnWidth, YColumnWidthLength};
use crate::declarations::contain::YContain;
use crate::declarations::container_name::YContainerName;
use crate::declarations::container_type::YContainerType;
use crate::declarations::content::{YContent, YContentCounterStyleSymbols, YContentItemAttr, YContentItemCounter, YContentItemCounters, YContentItemImage, YContentItemString, YContentItems};
use crate::declarations::counter_increment::{YCounterIncrement, YCounterIncrementCounter};
use crate::declarations::counter_reset::{YCounterReset, YCounterResetCounter};
use crate::declarations::csswide_keyword::YCSSWideKeyword;
use crate::declarations::cursor::{YCursor, YCursorImage};
use crate::declarations::custom::{YCustom, YCustomValueCSSWideKeyword, YCustomValueParsed, YCustomValueUnparsed};
use crate::declarations::direction::YDirection;
use crate::declarations::display::YDisplay;
use crate::declarations::empty_cells::YEmptyCells;
use crate::declarations::filter;
use crate::declarations::flex_basis::{YFlexBasis, YFlexBasisSize};
use crate::declarations::flex_direction::YFlexDirection;
use crate::declarations::flex_grow::YFlexGrow;
use crate::declarations::flex_shrink::YFlexShrink;
use crate::declarations::flex_wrap::YFlexWrap;
use crate::declarations::float::YFloat;
use crate::declarations::font_family::{YFontFamily, YFontFamilyGeneric, YFontFamilyName, YFontFamilyValues};
use crate::declarations::font_language_override::YFontLanguageOverride;
use crate::declarations::font_optical_sizing::YFontOpticalSizing;
use crate::declarations::font_size::{YFontSize, YFontSizeKeyword, YFontSizeLength};
use crate::declarations::font_stretch::{YFontStretch, YFontStretchKeyword, YFontStretchValue};
use crate::declarations::font_style::{YFontStyle, YFontStyleOblique};
use crate::declarations::font_synthesis_weight::YFontSynthesisWeight;
use crate::declarations::font_variant_caps::YFontVariantCaps;
use crate::declarations::font_variation_settings::{YFontVariationSetting, YFontVariationSettings};
use crate::declarations::font_weight::{YFontWeight, YFontWeightAbsolute};
use crate::declarations::grid_auto_columns::YGridAutoColumns;
use crate::declarations::grid_auto_flow::YGridAutoFlow;
use crate::declarations::grid_auto_rows::YGridAutoRows;
use crate::declarations::grid_column_end::YGridColumnEnd;
use crate::declarations::grid_column_start::YGridColumnStart;
use crate::declarations::grid_row_end::YGridRowEnd;
use crate::declarations::grid_row_start::YGridRowStart;
use crate::declarations::grid_template_areas::{YGridTemplateArea, YGridTemplateAreas, YGridTemplateAreaList};
use crate::declarations::grid_template_columns::YGridTemplateColumns;
use crate::declarations::grid_template_rows::YGridTemplateRows;
use crate::declarations::grid_template::{YGridTemplateLineNameListValue, YGridTemplateLineNameRepeat, YGridTemplateLineNames, YGridTemplateRepeatCountNumber, YGridTemplateSubgrid, YGridTemplateTrackList, YGridTemplateTrackListValue, YGridTemplateTrackListValueTrackRepeat};
use crate::declarations::height::YHeight;
use crate::declarations::image_rendering::YImageRendering;
use crate::declarations::images;
use crate::declarations::inline_size::YInlineSize;
use crate::declarations::inset_block_end::YInsetBlockEnd;
use crate::declarations::inset_block_start::YInsetBlockStart;
use crate::declarations::inset_inline_end::YInsetInlineEnd;
use crate::declarations::inset_inline_start::YInsetInlineStart;
use crate::declarations::inset::{YInsetAnchorContainingCalcFunction, YInsetAnchorFunction, YInsetAnchorSizeFunction, YInsetLengthPercentage};
use crate::declarations::isolation::YIsolation;
use crate::declarations::justify_content::YJustifyContent;
use crate::declarations::justify_items::YJustifyItems;
use crate::declarations::justify_self::YJustifySelf;
use crate::declarations::left::YLeft;
use crate::declarations::length::{YAbsoluteLength, YCharacterWidthLength, YContainerRelativeLength, YFontRelativeLength, YViewportPercentageLength};
use crate::declarations::letter_spacing::{YLetterSpacing};
use crate::declarations::line_break::YLineBreak;
use crate::declarations::line_height::YLineHeight;
use crate::declarations::list_style_image::YListStyleImage;
use crate::declarations::list_style_position::YListStylePosition;
use crate::declarations::list_style_type::{YListStyleType, YListStyleTypeName, YListStyleTypeString, YListStyleTypeSymbols};
use crate::declarations::margin_block_end::YMarginBlockEnd;
use crate::declarations::margin_block_start::YMarginBlockStart;
use crate::declarations::margin_bottom::YMarginBottom;
use crate::declarations::margin_inline_end::YMarginInlineEnd;
use crate::declarations::margin_inline_start::YMarginInlineStart;
use crate::declarations::margin_left::YMarginLeft;
use crate::declarations::margin_right::YMarginRight;
use crate::declarations::margin_top::YMarginTop;
use crate::declarations::margin::{YMarginAnchorContainingCalcFunction, YMarginAnchorSizeFunction};
use crate::declarations::mask_image::YMaskImage;
use crate::declarations::max_block_size::YMaxBlockSize;
use crate::declarations::max_height::YMaxHeight;
use crate::declarations::max_inline_size::YMaxInlineSize;
use crate::declarations::max_width::YMaxWidth;
use crate::declarations::min_block_size::YMinBlockSize;
use crate::declarations::min_height::YMinHeight;
use crate::declarations::min_inline_size::YMinInlineSize;
use crate::declarations::min_width::YMinWidth;
use crate::declarations::mix_blend_mode::YMixBlendMode;
use crate::declarations::number::YNumber;
use crate::declarations::object_fit::YObjectFit;
use crate::declarations::object_position::YObjectPosition;
use crate::declarations::opacity::YOpacity;
use crate::declarations::order::YOrder;
use crate::declarations::outline_color::YOutlineColor;
use crate::declarations::outline_offset::YOutlineOffset;
use crate::declarations::outline_style::YOutlineStyle;
use crate::declarations::outline_width::YOutlineWidth;
use crate::declarations::overflow_block::YOverflowBlock;
use crate::declarations::overflow_clip_margin::YOverflowClipMargin;
use crate::declarations::overflow_inline::YOverflowInline;
use crate::declarations::overflow_wrap::YOverflowWrap;
use crate::declarations::overflow_x::YOverflowX;
use crate::declarations::overflow_y::YOverflowY;
use crate::declarations::padding_block_end::YPaddingBlockEnd;
use crate::declarations::padding_block_start::YPaddingBlockStart;
use crate::declarations::padding_bottom::YPaddingBottom;
use crate::declarations::padding_inline_end::YPaddingInlineEnd;
use crate::declarations::padding_inline_start::YPaddingInlineStart;
use crate::declarations::padding_left::YPaddingLeft;
use crate::declarations::padding_right::YPaddingRight;
use crate::declarations::padding_top::YPaddingTop;
use crate::declarations::perspective::{YPerspective, YPerspectiveLength};
use crate::declarations::perspective_origin::YPerspectiveOrigin;
use crate::declarations::percentage::YPercentage;
use crate::declarations::pointer_events::YPointerEvents;
use crate::declarations::position::YPosition;
use crate::declarations::position_area::YPositionArea;
use crate::declarations::position_try_fallbacks::{YPositionTryFallbacks, YPositionTryFallbacksIdentAndOrTactic, YPositionTryFallbacksPositionArea};
use crate::declarations::quotes::{YQuotes, YQuotesQuoteList, YQuotesQuotePair};
use crate::declarations::resolution::YResolution;
use crate::declarations::right::YRight;
use crate::declarations::rotate::{YRotate, YRotate3D};
use crate::declarations::row_gap::{YRowGap, YRowGapLengthPercentage};
use crate::declarations::scale::{YScale, YScaleCoords};
use crate::declarations::servo_overflow_clip_box::YServoOverflowClipBox;
use crate::declarations::servo_top_layer::YServoTopLayer;
use crate::declarations::size;
use crate::declarations::table_layout::YTableLayout;
use crate::declarations::text_align::YTextAlign;
use crate::declarations::text_align_last::YTextAlignLast;
use crate::declarations::text_decoration_color::YTextDecorationColor;
use crate::declarations::text_decoration_line::YTextDecorationLine;
use crate::declarations::text_decoration_style::YTextDecorationStyle;
use crate::declarations::text_indent::YTextIndent;
use crate::declarations::text_justify::YTextJustify;
use crate::declarations::text_overflow::{YTextOverflow, YTextOverflowString};
use crate::declarations::text_rendering::YTextRendering;
use crate::declarations::text_shadow::{YTextShadow, YTextShadowShadow};
use crate::declarations::text_transform::YTextTransform;
use crate::declarations::text_wrap_mode::YTextWrapMode;
use crate::declarations::time::YTime;
use crate::declarations::top::YTop;
use crate::declarations::track_breadth::{YTrackBreadthFr, YTrackBreadthLengthPercentage};
use crate::declarations::track_size::{YTrackSizeFitContent, YTrackSizeMinmax};
use crate::declarations::transform::{
    YTransform,
    YTransformAccumulateMatrix,
    YTransformInterpolateMatrix,
    YTransformMatrix,
    YTransformMatrix3D,
    YTransformPerspective,
    YTransformPerspectiveLength,
    YTransformRotate,
    YTransformRotate3D,
    YTransformRotateX,
    YTransformRotateY,
    YTransformRotateZ,
    YTransformScale,
    YTransformScale3D,
    YTransformScaleX,
    YTransformScaleY,
    YTransformScaleZ,
    YTransformSkew,
    YTransformSkewX,
    YTransformSkewY,
    YTransformTranslate,
    YTransformTranslate3D,
    YTransformTranslateX,
    YTransformTranslateY,
    YTransformTranslateZ,
};
use crate::declarations::transform_origin::{YTransformOrigin, YSideHorizontalOriginComponent, YSideVerticalOriginComponent};
use crate::declarations::transform_style::YTransformStyle;
use crate::declarations::transition_behavior::YTransitionBehavior;
use crate::declarations::transition_delay::YTransitionDelay;
use crate::declarations::transition_duration::YTransitionDuration;
use crate::declarations::transition_property::{YTransitionProperty, YTransitionPropertyCustom, YTransitionPropertyNonCustom, YTransitionPropertyUnsupported};
use crate::declarations::transition_timing_function::YTransitionTimingFunction;
use crate::declarations::translate::{YTranslate, YTranslateCoords};
use crate::declarations::unicode_bidi::YUnicodeBidi;
use crate::declarations::view_transition_class::YViewTransitionClass;
use crate::declarations::view_transition_name::YViewTransitionName;
use crate::declarations::visibility::YVisibility;
use crate::declarations::webkit_text_security::YWebkitTextSecurity;
use crate::declarations::will_change::YWillChange;
use crate::declarations::with_variables::YWithVariables;
use crate::declarations::width::YWidth;
use crate::declarations::white_space_collapse::YWhiteSpaceCollapse;
use crate::declarations::word_break::YWordBreak;
use crate::declarations::word_spacing::YWordSpacing;

pub fn init(ruby: &Ruby, yass_module: &RModule) -> Result<(), Error> {
    let declarations_module = yass_module.define_module("Declarations")?;

    let align_items_class = declarations_module.define_class("AlignItems", ruby.class_object())?;
    align_items_class.define_method("value", method!(YAlignItems::value, 0))?;

    let alignment_baseline_class = declarations_module.define_class("AlignmentBaseline", ruby.class_object())?;
    alignment_baseline_class.define_method("value", method!(YAlignmentBaseline::value, 0))?;

    let angle_class = declarations_module.define_class("Angle", ruby.class_object())?;
    angle_class.define_method("calc?", method!(YAngle::was_calc, 0))?;
    angle_class.define_method("degrees", method!(YAngle::degrees, 0))?;
    angle_class.define_method("unit", method!(YAngle::unit, 0))?;

    let aspect_ratio_class = declarations_module.define_class("AspectRatio", ruby.class_object())?;
    aspect_ratio_class.define_method("auto?", method!(YAspectRatio::is_auto, 0))?;
    aspect_ratio_class.define_method("ratio?", method!(YAspectRatio::has_ratio, 0))?;
    aspect_ratio_class.define_method("numerator", method!(YAspectRatio::numerator, 0))?;
    aspect_ratio_class.define_method("denominator", method!(YAspectRatio::denominator, 0))?;

    let backface_visibility_class = declarations_module.define_class("BackfaceVisibility", ruby.class_object())?;
    backface_visibility_class.define_method("value", method!(YBackfaceVisibility::value, 0))?;

    let baseline_source_class = declarations_module.define_class("BaselineSource", ruby.class_object())?;
    baseline_source_class.define_method("value", method!(YBaselineSource::value, 0))?;

    let border_collapse_class = declarations_module.define_class("BorderCollapse", ruby.class_object())?;
    border_collapse_class.define_method("value", method!(YBorderCollapse::value, 0))?;

    let border_spacing_class = declarations_module.define_class("BorderSpacing", ruby.class_object())?;
    border_spacing_class.define_method("horizontal", method!(YBorderSpacing::horizontal, 0))?;
    border_spacing_class.define_method("vertical", method!(YBorderSpacing::vertical, 0))?;

    let border_image_outset_class = declarations_module.define_class("BorderImageOutset", ruby.class_object())?;
    border_image_outset_class.define_method("top", method!(YBorderImageOutset::top, 0))?;
    border_image_outset_class.define_method("right", method!(YBorderImageOutset::right, 0))?;
    border_image_outset_class.define_method("bottom", method!(YBorderImageOutset::bottom, 0))?;
    border_image_outset_class.define_method("left", method!(YBorderImageOutset::left, 0))?;

    let border_image_repeat_class = declarations_module.define_class("BorderImageRepeat", ruby.class_object())?;
    border_image_repeat_class.define_method("horizontal", method!(YBorderImageRepeat::horizontal, 0))?;
    border_image_repeat_class.define_method("vertical", method!(YBorderImageRepeat::vertical, 0))?;

    let border_image_slice_class = declarations_module.define_class("BorderImageSlice", ruby.class_object())?;
    border_image_slice_class.define_method("top", method!(YBorderImageSlice::top, 0))?;
    border_image_slice_class.define_method("right", method!(YBorderImageSlice::right, 0))?;
    border_image_slice_class.define_method("bottom", method!(YBorderImageSlice::bottom, 0))?;
    border_image_slice_class.define_method("left", method!(YBorderImageSlice::left, 0))?;
    border_image_slice_class.define_method("fill?", method!(YBorderImageSlice::fill, 0))?;

    let border_image_width_class = declarations_module.define_class("BorderImageWidth", ruby.class_object())?;
    border_image_width_class.define_method("top", method!(YBorderImageWidth::top, 0))?;
    border_image_width_class.define_method("right", method!(YBorderImageWidth::right, 0))?;
    border_image_width_class.define_method("bottom", method!(YBorderImageWidth::bottom, 0))?;
    border_image_width_class.define_method("left", method!(YBorderImageWidth::left, 0))?;
    let _border_image_width_auto_class = border_image_width_class.define_class("Auto", ruby.class_object())?;

    let box_sizing_class = declarations_module.define_class("BoxSizing", ruby.class_object())?;
    box_sizing_class.define_method("value", method!(YBoxSizing::value, 0))?;

    let caption_side_class = declarations_module.define_class("CaptionSide", ruby.class_object())?;
    caption_side_class.define_method("value", method!(YCaptionSide::value, 0))?;

    let clear_class = declarations_module.define_class("Clear", ruby.class_object())?;
    clear_class.define_method("value", method!(YClear::value, 0))?;

    let column_count_class = declarations_module.define_class("ColumnCount", ruby.class_object())?;
    let _column_count_auto_class = column_count_class.define_class("Auto", ruby.class_object())?;
    let column_count_integer_class = column_count_class.define_class("Integer", ruby.class_object())?;
    column_count_integer_class.define_method("value", method!(YColumnCountInteger::value, 0))?;

    let column_span_class = declarations_module.define_class("ColumnSpan", ruby.class_object())?;
    column_span_class.define_method("value", method!(YColumnSpan::value, 0))?;

    let contain_class = declarations_module.define_class("Contain", ruby.class_object())?;
    contain_class.define_method("values", method!(YContain::values, 0))?;
    contain_class.define_method("none?", method!(YContain::is_none, 0))?;
    contain_class.define_method("inline_size?", method!(YContain::is_inline_size, 0))?;
    contain_class.define_method("block_size?", method!(YContain::is_block_size, 0))?;
    contain_class.define_method("layout?", method!(YContain::is_layout, 0))?;
    contain_class.define_method("style?", method!(YContain::is_style, 0))?;
    contain_class.define_method("paint?", method!(YContain::is_paint, 0))?;
    contain_class.define_method("size?", method!(YContain::is_size, 0))?;
    contain_class.define_method("content?", method!(YContain::is_content, 0))?;
    contain_class.define_method("strict?", method!(YContain::is_strict, 0))?;

    let container_type_class = declarations_module.define_class("ContainerType", ruby.class_object())?;
    container_type_class.define_method("values", method!(YContainerType::values, 0))?;
    container_type_class.define_method("normal?", method!(YContainerType::is_normal, 0))?;
    container_type_class.define_method("inline_size?", method!(YContainerType::is_inline_size, 0))?;
    container_type_class.define_method("size?", method!(YContainerType::is_size, 0))?;
    container_type_class.define_method("scroll_state?", method!(YContainerType::is_scroll_state, 0))?;

    let direction_class = declarations_module.define_class("Direction", ruby.class_object())?;
    direction_class.define_method("value", method!(YDirection::value, 0))?;

    let display_class = declarations_module.define_class("Display", ruby.class_object())?;
    display_class.define_method("inside", method!(YDisplay::inside, 0))?;
    display_class.define_method("outside", method!(YDisplay::outside, 0))?;
    display_class.define_method("list_item?", method!(YDisplay::is_list_item, 0))?;
    display_class.define_method("contents?", method!(YDisplay::is_contents, 0))?;
    display_class.define_method("none?", method!(YDisplay::is_none, 0))?;
    display_class.define_method("inline_flow?", method!(YDisplay::is_inline_flow, 0))?;
    display_class.define_method("item_container?", method!(YDisplay::is_item_container, 0))?;
    display_class.define_method("line_participant?", method!(YDisplay::is_line_participant, 0))?;
    display_class.define_method("ruby_level_container?", method!(YDisplay::is_ruby_level_container, 0))?;
    display_class.define_method("ruby_type?", method!(YDisplay::is_ruby_type, 0))?;

    let empty_cells_class = declarations_module.define_class("EmptyCells", ruby.class_object())?;
    empty_cells_class.define_method("value", method!(YEmptyCells::value, 0))?;

    let flex_basis_class = declarations_module.define_class("FlexBasis", ruby.class_object())?;
    flex_basis_class.define_method("value", method!(YFlexBasis::value, 0))?;
    let _flex_basis_content_class = flex_basis_class.define_class("Content", ruby.class_object())?;
    let flex_basis_size_class = flex_basis_class.define_class("Size", ruby.class_object())?;
    flex_basis_size_class.define_method("value", method!(YFlexBasisSize::value, 0))?;

    let flex_direction_class = declarations_module.define_class("FlexDirection", ruby.class_object())?;
    flex_direction_class.define_method("value", method!(YFlexDirection::value, 0))?;

    let flex_wrap_class = declarations_module.define_class("FlexWrap", ruby.class_object())?;
    flex_wrap_class.define_method("value", method!(YFlexWrap::value, 0))?;

    let float_class = declarations_module.define_class("Float", ruby.class_object())?;
    float_class.define_method("value", method!(YFloat::value, 0))?;

    let font_language_override_class = declarations_module.define_class("FontLanguageOverride", ruby.class_object())?;
    font_language_override_class.define_method("normal?", method!(YFontLanguageOverride::is_normal, 0))?;
    font_language_override_class.define_method("value", method!(YFontLanguageOverride::value, 0))?;

    let font_optical_sizing_class = declarations_module.define_class("FontOpticalSizing", ruby.class_object())?;
    font_optical_sizing_class.define_method("value", method!(YFontOpticalSizing::value, 0))?;

    let font_stretch_class = declarations_module.define_class("FontStretch", ruby.class_object())?;
    font_stretch_class.define_method("value", method!(YFontStretch::value, 0))?;
    let font_stretch_stretch_class = font_stretch_class.define_class("Stretch", ruby.class_object())?;
    font_stretch_stretch_class.define_method("value", method!(YFontStretchValue::value, 0))?;
    let font_stretch_keyword_class = font_stretch_class.define_class("Keyword", ruby.class_object())?;
    font_stretch_keyword_class.define_method("value", method!(YFontStretchKeyword::value, 0))?;
    let _font_stretch_system_class = font_stretch_class.define_class("System", ruby.class_object())?;

    let font_style_class = declarations_module.define_class("FontStyle", ruby.class_object())?;
    font_style_class.define_method("value", method!(YFontStyle::value, 0))?;

    let _font_style_normal_class = font_style_class.define_class("Normal", ruby.class_object())?;
    let _font_style_italic_class = font_style_class.define_class("Italic", ruby.class_object())?;
    let font_style_oblique_class = font_style_class.define_class("Oblique", ruby.class_object())?;
    font_style_oblique_class.define_method("angle", method!(YFontStyleOblique::angle, 0))?;

    let _font_style_system_class = font_style_class.define_class("System", ruby.class_object())?;

    let font_synthesis_weight_class = declarations_module.define_class("FontSynthesisWeight", ruby.class_object())?;
    font_synthesis_weight_class.define_method("value", method!(YFontSynthesisWeight::value, 0))?;

    let font_variant_caps_class = declarations_module.define_class("FontVariantCaps", ruby.class_object())?;
    font_variant_caps_class.define_method("value", method!(YFontVariantCaps::value, 0))?;

    let font_weight_class = declarations_module.define_class("FontWeight", ruby.class_object())?;
    font_weight_class.define_method("value", method!(YFontWeight::value, 0))?;
    let font_weight_absolute_class = font_weight_class.define_class("Absolute", ruby.class_object())?;
    font_weight_absolute_class.define_method("value", method!(YFontWeightAbsolute::value, 0))?;
    let _font_weight_bolder_class = font_weight_class.define_class("Bolder", ruby.class_object())?;
    let _font_weight_lighter_class = font_weight_class.define_class("Lighter", ruby.class_object())?;
    let _font_weight_system_class = font_weight_class.define_class("System", ruby.class_object())?;

    let grid_auto_flow_class = declarations_module.define_class("GridAutoFlow", ruby.class_object())?;
    grid_auto_flow_class.define_method("value", method!(YGridAutoFlow::value, 0))?;
    grid_auto_flow_class.define_method("axis", method!(YGridAutoFlow::axis, 0))?;
    grid_auto_flow_class.define_method("dense?", method!(YGridAutoFlow::dense, 0))?;

    let image_rendering_class = declarations_module.define_class("ImageRendering", ruby.class_object())?;
    image_rendering_class.define_method("value", method!(YImageRendering::value, 0))?;

    let isolation_class = declarations_module.define_class("Isolation", ruby.class_object())?;
    isolation_class.define_method("value", method!(YIsolation::value, 0))?;

    let justify_items_class = declarations_module.define_class("JustifyItems", ruby.class_object())?;
    justify_items_class.define_method("value", method!(YJustifyItems::value, 0))?;

    let line_break_class = declarations_module.define_class("LineBreak", ruby.class_object())?;
    line_break_class.define_method("value", method!(YLineBreak::value, 0))?;

    let list_style_position_class = declarations_module.define_class("ListStylePosition", ruby.class_object())?;
    list_style_position_class.define_method("value", method!(YListStylePosition::value, 0))?;

    let mix_blend_mode_class = declarations_module.define_class("MixBlendMode", ruby.class_object())?;
    mix_blend_mode_class.define_method("value", method!(YMixBlendMode::value, 0))?;

    let object_fit_class = declarations_module.define_class("ObjectFit", ruby.class_object())?;
    object_fit_class.define_method("value", method!(YObjectFit::value, 0))?;

    let offset_path_class = declarations_module.define_class("OffsetPath", ruby.class_object())?;
    offset_path_class.define_method("value", method!(YOffsetPath::value, 0))?;

    let _offset_path_none_class = offset_path_class.define_class("None", ruby.class_object())?;

    let offset_path_coord_box_class = offset_path_class.define_class("CoordBox", ruby.class_object())?;
    offset_path_coord_box_class.define_method("value", method!(YOffsetPathCoordBox::value, 0))?;

    let offset_path_offset_path_class = offset_path_class.define_class("Function", ruby.class_object())?;
    offset_path_offset_path_class.define_method("path", method!(YOffsetPathFunction::path, 0))?;
    offset_path_offset_path_class.define_method("coord_box", method!(YOffsetPathFunction::coord_box, 0))?;

    let offset_path_ray_class = offset_path_class.define_class("Ray", ruby.class_object())?;
    offset_path_ray_class.define_method("angle", method!(YOffsetPathRay::angle, 0))?;
    offset_path_ray_class.define_method("size", method!(YOffsetPathRay::size, 0))?;
    offset_path_ray_class.define_method("contain?", method!(YOffsetPathRay::contain, 0))?;
    offset_path_ray_class.define_method("position", method!(YOffsetPathRay::position, 0))?;

    let offset_path_url_class = offset_path_class.define_class("Url", ruby.class_object())?;
    offset_path_url_class.define_method("value", method!(YOffsetPathUrl::value, 0))?;

    let _offset_path_position_auto_class = offset_path_class.define_class("PositionAuto", ruby.class_object())?;

    let offset_path_position_class = offset_path_class.define_class("Position", ruby.class_object())?;
    offset_path_position_class.define_method("horizontal", method!(YOffsetPathPosition::horizontal, 0))?;
    offset_path_position_class.define_method("vertical", method!(YOffsetPathPosition::vertical, 0))?;

    let object_position_class = declarations_module.define_class("ObjectPosition", ruby.class_object())?;
    object_position_class.define_method("horizontal", method!(YObjectPosition::horizontal, 0))?;
    object_position_class.define_method("vertical", method!(YObjectPosition::vertical, 0))?;

    let opacity_class = declarations_module.define_class("Opacity", ruby.class_object())?;
    opacity_class.define_method("value", method!(YOpacity::value, 0))?;

    let order_class = declarations_module.define_class("Order", ruby.class_object())?;
    order_class.define_method("value", method!(YOrder::value, 0))?;

    let outline_style_class = declarations_module.define_class("OutlineStyle", ruby.class_object())?;
    outline_style_class.define_method("value", method!(YOutlineStyle::value, 0))?;

    let overflow_wrap_class = declarations_module.define_class("OverflowWrap", ruby.class_object())?;
    overflow_wrap_class.define_method("value", method!(YOverflowWrap::value, 0))?;

    let pointer_events_class = declarations_module.define_class("PointerEvents", ruby.class_object())?;
    pointer_events_class.define_method("value", method!(YPointerEvents::value, 0))?;

    let position_class = declarations_module.define_class("Position", ruby.class_object())?;
    position_class.define_method("value", method!(YPosition::value, 0))?;

    let position_area_class = declarations_module.define_class("PositionArea", ruby.class_object())?;
    position_area_class.define_method("first", method!(YPositionArea::first, 0))?;
    position_area_class.define_method("second", method!(YPositionArea::second, 0))?;
    position_area_class.define_method("is_none", method!(YPositionArea::is_none, 0))?;

    let servo_overflow_clip_box_class = declarations_module.define_class("ServoOverflowClipBox", ruby.class_object())?;
    servo_overflow_clip_box_class.define_method("value", method!(YServoOverflowClipBox::value, 0))?;

    let servo_top_layer_class = declarations_module.define_class("ServoTopLayer", ruby.class_object())?;
    servo_top_layer_class.define_method("value", method!(YServoTopLayer::value, 0))?;

    let table_layout_class = declarations_module.define_class("TableLayout", ruby.class_object())?;
    table_layout_class.define_method("value", method!(YTableLayout::value, 0))?;

    let text_align_class = declarations_module.define_class("TextAlign", ruby.class_object())?;
    text_align_class.define_method("value", method!(YTextAlign::value, 0))?;

    let text_align_last_class = declarations_module.define_class("TextAlignLast", ruby.class_object())?;
    text_align_last_class.define_method("value", method!(YTextAlignLast::value, 0))?;

    let text_decoration_line_class = declarations_module.define_class("TextDecorationLine", ruby.class_object())?;
    text_decoration_line_class.define_method("values", method!(YTextDecorationLine::values, 0))?;
    text_decoration_line_class.define_method("none?", method!(YTextDecorationLine::is_none, 0))?;
    text_decoration_line_class.define_method("underline?", method!(YTextDecorationLine::is_underline, 0))?;
    text_decoration_line_class.define_method("overline?", method!(YTextDecorationLine::is_overline, 0))?;
    text_decoration_line_class.define_method("line_through?", method!(YTextDecorationLine::is_line_through, 0))?;
    text_decoration_line_class.define_method("blink?", method!(YTextDecorationLine::is_blink, 0))?;
    text_decoration_line_class.define_method("spelling_error?", method!(YTextDecorationLine::is_spelling_error, 0))?;
    text_decoration_line_class.define_method("grammar_error?", method!(YTextDecorationLine::is_grammar_error, 0))?;

    let text_decoration_style_class = declarations_module.define_class("TextDecorationStyle", ruby.class_object())?;
    text_decoration_style_class.define_method("value", method!(YTextDecorationStyle::value, 0))?;

    let text_overflow_class = declarations_module.define_class("TextOverflow", ruby.class_object())?;
    text_overflow_class.define_method("first", method!(YTextOverflow::first, 0))?;
    text_overflow_class.define_method("second", method!(YTextOverflow::second, 0))?;
    text_overflow_class.define_method("sides_are_logical?", method!(YTextOverflow::sides_are_logical, 0))?;

    let _text_overflow_clip_class = text_overflow_class.define_class("Clip", ruby.class_object())?;

    let _text_overflow_ellipsis_class = text_overflow_class.define_class("Ellipsis", ruby.class_object())?;

    let text_overflow_string_class = text_overflow_class.define_class("String", ruby.class_object())?;
    text_overflow_string_class.define_method("value", method!(YTextOverflowString::value, 0))?;

    let text_justify_class = declarations_module.define_class("TextJustify", ruby.class_object())?;
    text_justify_class.define_method("value", method!(YTextJustify::value, 0))?;

    let text_rendering_class = declarations_module.define_class("TextRendering", ruby.class_object())?;
    text_rendering_class.define_method("value", method!(YTextRendering::value, 0))?;

    let text_transform_class = declarations_module.define_class("TextTransform", ruby.class_object())?;
    text_transform_class.define_method("values", method!(YTextTransform::values, 0))?;

    let text_wrap_mode_class = declarations_module.define_class("TextWrapMode", ruby.class_object())?;
    text_wrap_mode_class.define_method("value", method!(YTextWrapMode::value, 0))?;

    let transform_style_class = declarations_module.define_class("TransformStyle", ruby.class_object())?;
    transform_style_class.define_method("value", method!(YTransformStyle::value, 0))?;

    let unicode_bidi_class = declarations_module.define_class("UnicodeBidi", ruby.class_object())?;
    unicode_bidi_class.define_method("value", method!(YUnicodeBidi::value, 0))?;

    let visibility_class = declarations_module.define_class("Visibility", ruby.class_object())?;
    visibility_class.define_method("value", method!(YVisibility::value, 0))?;

    let webkit_text_security_class = declarations_module.define_class("WebkitTextSecurity", ruby.class_object())?;
    webkit_text_security_class.define_method("value", method!(YWebkitTextSecurity::value, 0))?;

    let white_space_collapse_class = declarations_module.define_class("WhiteSpaceCollapse", ruby.class_object())?;
    white_space_collapse_class.define_method("value", method!(YWhiteSpaceCollapse::value, 0))?;

    let word_break_class = declarations_module.define_class("WordBreak", ruby.class_object())?;
    word_break_class.define_method("value", method!(YWordBreak::value, 0))?;

    let writing_mode_class = declarations_module.define_class("WritingMode", ruby.class_object())?;

    let zindex_class = declarations_module.define_class("ZIndex", ruby.class_object())?;

    let zoom_class = declarations_module.define_class("Zoom", ruby.class_object())?;

    let align_content_class = declarations_module.define_class("AlignContent", ruby.class_object())?;
    align_content_class.define_method("value", method!(YAlignContent::value, 0))?;

    let justify_content_class = declarations_module.define_class("JustifyContent", ruby.class_object())?;
    justify_content_class.define_method("value", method!(YJustifyContent::value, 0))?;

    let flex_grow_class = declarations_module.define_class("FlexGrow", ruby.class_object())?;
    flex_grow_class.define_method("value", method!(YFlexGrow::value, 0))?;

    let flex_shrink_class = declarations_module.define_class("FlexShrink", ruby.class_object())?;
    flex_shrink_class.define_method("value", method!(YFlexShrink::value, 0))?;

    let align_self_class = declarations_module.define_class("AlignSelf", ruby.class_object())?;
    align_self_class.define_method("value", method!(YAlignSelf::value, 0))?;

    let justify_self_class = declarations_module.define_class("JustifySelf", ruby.class_object())?;
    justify_self_class.define_method("value", method!(YJustifySelf::value, 0))?;

    let overflow_block_class = declarations_module.define_class("OverflowBlock", ruby.class_object())?;
    overflow_block_class.define_method("value", method!(YOverflowBlock::value, 0))?;

    let overflow_inline_class = declarations_module.define_class("OverflowInline", ruby.class_object())?;
    overflow_inline_class.define_method("value", method!(YOverflowInline::value, 0))?;

    let overflow_x_class = declarations_module.define_class("OverflowX", ruby.class_object())?;
    overflow_x_class.define_method("value", method!(YOverflowX::value, 0))?;

    let overflow_y_class = declarations_module.define_class("OverflowY", ruby.class_object())?;
    overflow_y_class.define_method("value", method!(YOverflowY::value, 0))?;

    let border_block_end_style_class = declarations_module.define_class("BorderBlockEndStyle", ruby.class_object())?;
    border_block_end_style_class.define_method("value", method!(YBorderBlockEndStyle::value, 0))?;

    let border_block_start_style_class = declarations_module.define_class("BorderBlockStartStyle", ruby.class_object())?;
    border_block_start_style_class.define_method("value", method!(YBorderBlockStartStyle::value, 0))?;

    let border_bottom_style_class = declarations_module.define_class("BorderBottomStyle", ruby.class_object())?;
    border_bottom_style_class.define_method("value", method!(YBorderBottomStyle::value, 0))?;

    let border_bottom_left_radius_class = declarations_module.define_class("BorderBottomLeftRadius", ruby.class_object())?;
    border_bottom_left_radius_class.define_method("width", method!(YBorderBottomLeftRadius::width, 0))?;
    border_bottom_left_radius_class.define_method("height", method!(YBorderBottomLeftRadius::height, 0))?;

    let border_bottom_right_radius_class = declarations_module.define_class("BorderBottomRightRadius", ruby.class_object())?;
    border_bottom_right_radius_class.define_method("width", method!(YBorderBottomRightRadius::width, 0))?;
    border_bottom_right_radius_class.define_method("height", method!(YBorderBottomRightRadius::height, 0))?;

    let border_end_end_radius_class = declarations_module.define_class("BorderEndEndRadius", ruby.class_object())?;
    border_end_end_radius_class.define_method("width", method!(YBorderEndEndRadius::width, 0))?;
    border_end_end_radius_class.define_method("height", method!(YBorderEndEndRadius::height, 0))?;

    let border_end_start_radius_class = declarations_module.define_class("BorderEndStartRadius", ruby.class_object())?;
    border_end_start_radius_class.define_method("width", method!(YBorderEndStartRadius::width, 0))?;
    border_end_start_radius_class.define_method("height", method!(YBorderEndStartRadius::height, 0))?;

    let border_start_end_radius_class = declarations_module.define_class("BorderStartEndRadius", ruby.class_object())?;
    border_start_end_radius_class.define_method("width", method!(YBorderStartEndRadius::width, 0))?;
    border_start_end_radius_class.define_method("height", method!(YBorderStartEndRadius::height, 0))?;

    let border_start_start_radius_class = declarations_module.define_class("BorderStartStartRadius", ruby.class_object())?;
    border_start_start_radius_class.define_method("width", method!(YBorderStartStartRadius::width, 0))?;
    border_start_start_radius_class.define_method("height", method!(YBorderStartStartRadius::height, 0))?;

    let border_top_left_radius_class = declarations_module.define_class("BorderTopLeftRadius", ruby.class_object())?;
    border_top_left_radius_class.define_method("width", method!(YBorderTopLeftRadius::width, 0))?;
    border_top_left_radius_class.define_method("height", method!(YBorderTopLeftRadius::height, 0))?;

    let border_top_right_radius_class = declarations_module.define_class("BorderTopRightRadius", ruby.class_object())?;
    border_top_right_radius_class.define_method("width", method!(YBorderTopRightRadius::width, 0))?;
    border_top_right_radius_class.define_method("height", method!(YBorderTopRightRadius::height, 0))?;

    let border_inline_end_style_class = declarations_module.define_class("BorderInlineEndStyle", ruby.class_object())?;
    border_inline_end_style_class.define_method("value", method!(YBorderInlineEndStyle::value, 0))?;

    let border_inline_start_style_class = declarations_module.define_class("BorderInlineStartStyle", ruby.class_object())?;
    border_inline_start_style_class.define_method("value", method!(YBorderInlineStartStyle::value, 0))?;

    let border_left_style_class = declarations_module.define_class("BorderLeftStyle", ruby.class_object())?;
    border_left_style_class.define_method("value", method!(YBorderLeftStyle::value, 0))?;

    let border_right_style_class = declarations_module.define_class("BorderRightStyle", ruby.class_object())?;
    border_right_style_class.define_method("value", method!(YBorderRightStyle::value, 0))?;

    let border_top_style_class = declarations_module.define_class("BorderTopStyle", ruby.class_object())?;
    border_top_style_class.define_method("value", method!(YBorderTopStyle::value, 0))?;

    let animation_composition_class = declarations_module.define_class("AnimationComposition", ruby.class_object())?;
    animation_composition_class.define_method("values", method!(YAnimationComposition::values, 0))?;

    let animation_delay_class = declarations_module.define_class("AnimationDelay", ruby.class_object())?;
    animation_delay_class.define_method("values", method!(YAnimationDelay::values, 0))?;

    let animation_direction_class = declarations_module.define_class("AnimationDirection", ruby.class_object())?;
    animation_direction_class.define_method("values", method!(YAnimationDirection::values, 0))?;

    let animation_duration_class = declarations_module.define_class("AnimationDuration", ruby.class_object())?;
    animation_duration_class.define_method("values", method!(YAnimationDuration::values, 0))?;

    let animation_fill_mode_class = declarations_module.define_class("AnimationFillMode", ruby.class_object())?;
    animation_fill_mode_class.define_method("values", method!(YAnimationFillMode::values, 0))?;

    let animation_iteration_count_class = declarations_module.define_class("AnimationIterationCount", ruby.class_object())?;
    animation_iteration_count_class.define_method("values", method!(YAnimationIterationCount::values, 0))?;

    let animation_name_class = declarations_module.define_class("AnimationName", ruby.class_object())?;
    animation_name_class.define_method("values", method!(YAnimationName::values, 0))?;

    let animation_play_state_class = declarations_module.define_class("AnimationPlayState", ruby.class_object())?;
    animation_play_state_class.define_method("values", method!(YAnimationPlayState::values, 0))?;

    let animation_range_end_class = declarations_module.define_class("AnimationRangeEnd", ruby.class_object())?;
    animation_range_end_class.define_method("values", method!(YAnimationRangeEnd::values, 0))?;

    let animation_range_start_class = declarations_module.define_class("AnimationRangeStart", ruby.class_object())?;
    animation_range_start_class.define_method("values", method!(YAnimationRangeStart::values, 0))?;

    let animation_timeline_class = declarations_module.define_class("AnimationTimeline", ruby.class_object())?;
    animation_timeline_class.define_method("values", method!(YAnimationTimeline::values, 0))?;

    let animation_timing_function_class = declarations_module.define_class("AnimationTimingFunction", ruby.class_object())?;
    animation_timing_function_class.define_method("values", method!(YAnimationTimingFunction::values, 0))?;

    let backdrop_filter_class = declarations_module.define_class("BackdropFilter", ruby.class_object())?;
    backdrop_filter_class.define_method("values", method!(YBackdropFilter::values, 0))?;

    let background_attachment_class = declarations_module.define_class("BackgroundAttachment", ruby.class_object())?;
    background_attachment_class.define_method("values", method!(YBackgroundAttachment::values, 0))?;

    let background_blend_mode_class = declarations_module.define_class("BackgroundBlendMode", ruby.class_object())?;
    background_blend_mode_class.define_method("values", method!(YBackgroundBlendMode::values, 0))?;

    let background_clip_class = declarations_module.define_class("BackgroundClip", ruby.class_object())?;
    background_clip_class.define_method("values", method!(YBackgroundClip::values, 0))?;

    let background_image_class = declarations_module.define_class("BackgroundImage", ruby.class_object())?;
    background_image_class.define_method("values", method!(YBackgroundImage::values, 0))?;

    let background_origin_class = declarations_module.define_class("BackgroundOrigin", ruby.class_object())?;
    background_origin_class.define_method("values", method!(YBackgroundOrigin::values, 0))?;

    let background_position_x_class = declarations_module.define_class("BackgroundPositionX", ruby.class_object())?;
    background_position_x_class.define_method("values", method!(YBackgroundPositionX::values, 0))?;

    let background_position_y_class = declarations_module.define_class("BackgroundPositionY", ruby.class_object())?;
    background_position_y_class.define_method("values", method!(YBackgroundPositionY::values, 0))?;

    let background_repeat_class = declarations_module.define_class("BackgroundRepeat", ruby.class_object())?;
    background_repeat_class.define_method("values", method!(YBackgroundRepeat::values, 0))?;

    let background_repeat_value_class = declarations_module.define_class("BackgroundRepeatValue", ruby.class_object())?;
    background_repeat_value_class.define_method("horizontal", method!(YBackgroundRepeatValue::horizontal, 0))?;
    background_repeat_value_class.define_method("vertical", method!(YBackgroundRepeatValue::vertical, 0))?;

    let background_size_class = declarations_module.define_class("BackgroundSize", ruby.class_object())?;
    background_size_class.define_method("values", method!(YBackgroundSize::values, 0))?;

    let background_size_explicit_size_class = background_size_class.define_class("ExplicitSize", ruby.class_object())?;
    background_size_explicit_size_class.define_method("width", method!(YBackgroundSizeExplicitSize::width, 0))?;
    background_size_explicit_size_class.define_method("height", method!(YBackgroundSizeExplicitSize::height, 0))?;

    let _background_size_cover_class = background_size_class.define_class("Cover", ruby.class_object())?;

    let _background_size_contain_class = background_size_class.define_class("Contain", ruby.class_object())?;

    let _background_size_auto_class = background_size_class.define_class("Auto", ruby.class_object())?;

    let baseline_shift_class = declarations_module.define_class("BaselineShift", ruby.class_object())?;
    let baseline_shift_keyword_class = baseline_shift_class.define_class("Keyword", ruby.class_object())?;
    baseline_shift_keyword_class.define_method("value", method!(YBaselineShiftKeyword::value, 0))?;
    let baseline_shift_length_class = baseline_shift_class.define_class("Length", ruby.class_object())?;
    baseline_shift_length_class.define_method("value", method!(YBaselineShiftLength::value, 0))?;

    let box_shadow_class = declarations_module.define_class("BoxShadow", ruby.class_object())?;
    box_shadow_class.define_method("values", method!(YBoxShadow::values, 0))?;

    let box_shadow_shadow_class = box_shadow_class.define_class("Shadow", ruby.class_object())?;
    box_shadow_shadow_class.define_method("color", method!(YBoxShadowShadow::color, 0))?;
    box_shadow_shadow_class.define_method("horizontal", method!(YBoxShadowShadow::horizontal, 0))?;
    box_shadow_shadow_class.define_method("vertical", method!(YBoxShadowShadow::vertical, 0))?;
    box_shadow_shadow_class.define_method("blur", method!(YBoxShadowShadow::blur, 0))?;
    box_shadow_shadow_class.define_method("spread", method!(YBoxShadowShadow::spread, 0))?;
    box_shadow_shadow_class.define_method("inset?", method!(YBoxShadowShadow::is_inset, 0))?;

    let caret_color_class = declarations_module.define_class("CaretColor", ruby.class_object())?;
    caret_color_class.define_method("color", method!(YCaretColor::color, 0))?;

    let channel_keyword_class = declarations_module.define_class("ChannelKeyword", ruby.class_object())?;
    channel_keyword_class.define_method("value", method!(YChannelKeyword::value, 0))?;

    let clip_class = declarations_module.define_class("Clip", ruby.class_object())?;
    clip_class.define_method("value", method!(YClip::value, 0))?;

    let _clip_auto_class = clip_class.define_class("Auto", ruby.class_object())?;

    let clip_rect_class = clip_class.define_class("Rect", ruby.class_object())?;
    clip_rect_class.define_method("top", method!(YClipRect::top, 0))?;
    clip_rect_class.define_method("right", method!(YClipRect::right, 0))?;
    clip_rect_class.define_method("bottom", method!(YClipRect::bottom, 0))?;
    clip_rect_class.define_method("left", method!(YClipRect::left, 0))?;

    let clip_length_class = clip_class.define_class("Length", ruby.class_object())?;
    clip_length_class.define_method("value", method!(YClipLength::value, 0))?;

    let _clip_length_auto_class = clip_class.define_class("LengthAuto", ruby.class_object())?;

    let color_class = declarations_module.define_class("Color", ruby.class_object())?;
    color_class.define_method("color", method!(YColor::color, 0))?;

    let color_scheme_class = declarations_module.define_class("ColorScheme", ruby.class_object())?;
    color_scheme_class.define_method("values", method!(YColorScheme::values, 0))?;

    let column_width_class = declarations_module.define_class("ColumnWidth", ruby.class_object())?;
    column_width_class.define_method("value", method!(YColumnWidth::value, 0))?;
    let _column_width_auto_class = column_width_class.define_class("Auto", ruby.class_object())?;
    let column_width_length_class = column_width_class.define_class("Length", ruby.class_object())?;
    column_width_length_class.define_method("value", method!(YColumnWidthLength::value, 0))?;

    let container_name_class = declarations_module.define_class("ContainerName", ruby.class_object())?;
    container_name_class.define_method("values", method!(YContainerName::values, 0))?;
    container_name_class.define_method("none?", method!(YContainerName::is_none, 0))?;

    let content_class = declarations_module.define_class("Content", ruby.class_object())?;
    content_class.define_method("value", method!(YContent::value, 0))?;
    let _content_normal_class = content_class.define_class("Normal", ruby.class_object())?;
    let _content_none_class = content_class.define_class("None", ruby.class_object())?;

    let content_items_class = content_class.define_class("Items", ruby.class_object())?;
    content_items_class.define_method("items", method!(YContentItems::items, 0))?;
    content_items_class.define_method("alt_start", method!(YContentItems::alt_start, 0))?;

    let content_item_module = content_class.define_module("Item")?;

    let content_item_string_class = content_item_module.define_class("String", ruby.class_object())?;
    content_item_string_class.define_method("value", method!(YContentItemString::value, 0))?;

    let content_item_counter_class = content_item_module.define_class("Counter", ruby.class_object())?;
    content_item_counter_class.define_method("name", method!(YContentItemCounter::name, 0))?;
    content_item_counter_class.define_method("style", method!(YContentItemCounter::style, 0))?;

    let content_item_counters_class = content_item_module.define_class("Counters", ruby.class_object())?;
    content_item_counters_class.define_method("name", method!(YContentItemCounters::name, 0))?;
    content_item_counters_class.define_method("separator", method!(YContentItemCounters::separator, 0))?;
    content_item_counters_class.define_method("style", method!(YContentItemCounters::style, 0))?;

    let content_item_attr_class = content_item_module.define_class("Attr", ruby.class_object())?;
    content_item_attr_class.define_method("namespace_prefix", method!(YContentItemAttr::namespace_prefix, 0))?;
    content_item_attr_class.define_method("namespace_url", method!(YContentItemAttr::namespace_url, 0))?;
    content_item_attr_class.define_method("attribute", method!(YContentItemAttr::attribute, 0))?;
    content_item_attr_class.define_method("fallback", method!(YContentItemAttr::fallback, 0))?;

    let content_item_image_class = content_item_module.define_class("Image", ruby.class_object())?;
    content_item_image_class.define_method("image", method!(YContentItemImage::image, 0))?;

    let content_counter_style_module = content_class.define_module("CounterStyle")?;
    let content_counter_style_symbols_class = content_counter_style_module.define_class("Symbols", ruby.class_object())?;
    content_counter_style_symbols_class.define_method("kind", method!(YContentCounterStyleSymbols::kind, 0))?;
    content_counter_style_symbols_class.define_method("symbols", method!(YContentCounterStyleSymbols::symbols, 0))?;

    let _ = (content_items_class, content_item_module, content_item_string_class, content_item_counter_class, content_item_counters_class, content_item_attr_class, content_item_image_class, content_counter_style_module, content_counter_style_symbols_class);

    let counter_increment_class = declarations_module.define_class("CounterIncrement", ruby.class_object())?;
    counter_increment_class.define_method("values", method!(YCounterIncrement::values, 0))?;

    let counter_increment_counter_class = counter_increment_class.define_class("Counter", ruby.class_object())?;
    counter_increment_counter_class.define_method("name", method!(YCounterIncrementCounter::name, 0))?;
    counter_increment_counter_class.define_method("value", method!(YCounterIncrementCounter::value, 0))?;
    counter_increment_counter_class.define_method("reversed?", method!(YCounterIncrementCounter::reversed, 0))?;

    let counter_reset_class = declarations_module.define_class("CounterReset", ruby.class_object())?;
    counter_reset_class.define_method("values", method!(YCounterReset::values, 0))?;

    let counter_reset_counter_class = counter_reset_class.define_class("Counter", ruby.class_object())?;
    counter_reset_counter_class.define_method("name", method!(YCounterResetCounter::name, 0))?;
    counter_reset_counter_class.define_method("value", method!(YCounterResetCounter::value, 0))?;
    counter_reset_counter_class.define_method("reversed?", method!(YCounterResetCounter::reversed, 0))?;

    let cursor_class = declarations_module.define_class("Cursor", ruby.class_object())?;
    cursor_class.define_method("images", method!(YCursor::images, 0))?;
    cursor_class.define_method("keyword", method!(YCursor::keyword, 0))?;
    let cursor_image_class = cursor_class.define_class("Image", ruby.class_object())?;
    cursor_image_class.define_method("image", method!(YCursorImage::image, 0))?;
    cursor_image_class.define_method("hotspot?", method!(YCursorImage::has_hotspot, 0))?;
    cursor_image_class.define_method("hotspot_x", method!(YCursorImage::hotspot_x, 0))?;
    cursor_image_class.define_method("hotspot_y", method!(YCursorImage::hotspot_y, 0))?;

    let font_family_class = declarations_module.define_class("FontFamily", ruby.class_object())?;
    font_family_class.define_method("value", method!(YFontFamily::value, 0))?;

    let font_family_values_class = font_family_class.define_class("Values", ruby.class_object())?;
    font_family_values_class.define_method("values", method!(YFontFamilyValues::values, 0))?;

    let _font_family_system_class = font_family_class.define_class("System", ruby.class_object())?;

    let font_family_name_class = font_family_class.define_class("Name", ruby.class_object())?;
    font_family_name_class.define_method("value", method!(YFontFamilyName::value, 0))?;
    font_family_name_class.define_method("syntax", method!(YFontFamilyName::syntax, 0))?;

    let font_family_generic_class = font_family_class.define_class("Generic", ruby.class_object())?;
    font_family_generic_class.define_method("value", method!(YFontFamilyGeneric::value, 0))?;

    let font_size_class = declarations_module.define_class("FontSize", ruby.class_object())?;
    font_size_class.define_method("value", method!(YFontSize::value, 0))?;

    let font_size_length_class = font_size_class.define_class("Length", ruby.class_object())?;
    font_size_length_class.define_method("value", method!(YFontSizeLength::value, 0))?;

    let font_size_keyword_class = font_size_class.define_class("Keyword", ruby.class_object())?;
    font_size_keyword_class.define_method("keyword", method!(YFontSizeKeyword::keyword, 0))?;
    font_size_keyword_class.define_method("factor", method!(YFontSizeKeyword::factor, 0))?;

    let _font_size_smaller_class = font_size_class.define_class("Smaller", ruby.class_object())?;

    let _font_size_larger_class = font_size_class.define_class("Larger", ruby.class_object())?;

    let _font_size_system_class = font_size_class.define_class("System", ruby.class_object())?;

    let font_variation_settings_class = declarations_module.define_class("FontVariationSettings", ruby.class_object())?;
    font_variation_settings_class.define_method("values", method!(YFontVariationSettings::values, 0))?;
    font_variation_settings_class.define_method("normal?", method!(YFontVariationSettings::is_normal, 0))?;

    let font_variation_setting_class = font_variation_settings_class.define_class("Setting", ruby.class_object())?;
    font_variation_setting_class.define_method("tag", method!(YFontVariationSetting::tag, 0))?;
    font_variation_setting_class.define_method("value", method!(YFontVariationSetting::value, 0))?;

    let grid_template_areas_class = declarations_module.define_class("GridTemplateAreas", ruby.class_object())?;
    grid_template_areas_class.define_method("areas", method!(YGridTemplateAreas::areas, 0))?;

    let _grid_template_areas_none_class = grid_template_areas_class.define_class("None", ruby.class_object())?;

    let grid_template_area_list_class = grid_template_areas_class.define_class("AreaList", ruby.class_object())?;
    grid_template_area_list_class.define_method("width", method!(YGridTemplateAreaList::width, 0))?;
    grid_template_area_list_class.define_method("strings", method!(YGridTemplateAreaList::strings, 0))?;
    grid_template_area_list_class.define_method("areas", method!(YGridTemplateAreaList::areas, 0))?;

    let grid_template_area_class = grid_template_areas_class.define_class("Area", ruby.class_object())?;
    grid_template_area_class.define_method("name", method!(YGridTemplateArea::name, 0))?;
    grid_template_area_class.define_method("row_start", method!(YGridTemplateArea::row_start, 0))?;
    grid_template_area_class.define_method("row_end", method!(YGridTemplateArea::row_end, 0))?;
    grid_template_area_class.define_method("column_start", method!(YGridTemplateArea::column_start, 0))?;
    grid_template_area_class.define_method("column_end", method!(YGridTemplateArea::column_end, 0))?;

    let letter_spacing_class = declarations_module.define_class("LetterSpacing", ruby.class_object())?;
    letter_spacing_class.define_method("value", method!(YLetterSpacing::value, 0))?;

    let _letter_spacing_normal_class = letter_spacing_class.define_class("Normal", ruby.class_object())?;

    let line_height_class = declarations_module.define_class("LineHeight", ruby.class_object())?;
    line_height_class.define_method("value", method!(YLineHeight::value, 0))?;

    let _line_height_normal_class = line_height_class.define_class("Normal", ruby.class_object())?;

    let list_style_type_class = declarations_module.define_class("ListStyleType", ruby.class_object())?;
    list_style_type_class.define_method("value", method!(YListStyleType::value, 0))?;
    list_style_type_class.define_method("bullet?", method!(YListStyleType::bullet, 0))?;

    let _list_style_type_none_class = list_style_type_class.define_class("None", ruby.class_object())?;

    let list_style_type_name_class = list_style_type_class.define_class("Name", ruby.class_object())?;
    list_style_type_name_class.define_method("value", method!(YListStyleTypeName::value, 0))?;

    let list_style_type_string_class = list_style_type_class.define_class("String", ruby.class_object())?;
    list_style_type_string_class.define_method("value", method!(YListStyleTypeString::value, 0))?;

    let list_style_type_symbols_class = list_style_type_class.define_class("Symbols", ruby.class_object())?;
    list_style_type_symbols_class.define_method("type", method!(YListStyleTypeSymbols::ty, 0))?;
    list_style_type_symbols_class.define_method("symbols", method!(YListStyleTypeSymbols::symbols, 0))?;

    let mask_image_class = declarations_module.define_class("MaskImage", ruby.class_object())?;
    mask_image_class.define_method("values", method!(YMaskImage::values, 0))?;

    let outline_offset_class = declarations_module.define_class("OutlineOffset", ruby.class_object())?;
    outline_offset_class.define_method("value", method!(YOutlineOffset::value, 0))?;

    let overflow_clip_margin_class = declarations_module.define_class("OverflowClipMargin", ruby.class_object())?;
    overflow_clip_margin_class.define_method("offset", method!(YOverflowClipMargin::offset, 0))?;
    overflow_clip_margin_class.define_method("visual_box", method!(YOverflowClipMargin::visual_box, 0))?;

    let perspective_class = declarations_module.define_class("Perspective", ruby.class_object())?;
    perspective_class.define_method("value", method!(YPerspective::value, 0))?;

    let _perspective_none_class = perspective_class.define_class("None", ruby.class_object())?;

    let perspective_length_class = perspective_class.define_class("Length", ruby.class_object())?;
    perspective_length_class.define_method("value", method!(YPerspectiveLength::value, 0))?;

    let position_try_fallbacks_class = declarations_module.define_class("PositionTryFallbacks", ruby.class_object())?;
    position_try_fallbacks_class.define_method("values", method!(YPositionTryFallbacks::values, 0))?;
    position_try_fallbacks_class.define_method("is_none", method!(YPositionTryFallbacks::is_none, 0))?;

    let position_try_fallbacks_ident_and_or_tactic_class = position_try_fallbacks_class.define_class("IdentAndOrTactic", ruby.class_object())?;
    position_try_fallbacks_ident_and_or_tactic_class.define_method("ident", method!(YPositionTryFallbacksIdentAndOrTactic::ident, 0))?;
    position_try_fallbacks_ident_and_or_tactic_class.define_method("has_ident?", method!(YPositionTryFallbacksIdentAndOrTactic::has_ident, 0))?;
    position_try_fallbacks_ident_and_or_tactic_class.define_method("try_tactics", method!(YPositionTryFallbacksIdentAndOrTactic::try_tactics, 0))?;
    position_try_fallbacks_ident_and_or_tactic_class.define_method("has_try_tactics?", method!(YPositionTryFallbacksIdentAndOrTactic::has_try_tactics, 0))?;

    let position_try_fallbacks_position_area_class = position_try_fallbacks_class.define_class("PositionArea", ruby.class_object())?;
    position_try_fallbacks_position_area_class.define_method("first", method!(YPositionTryFallbacksPositionArea::first, 0))?;
    position_try_fallbacks_position_area_class.define_method("second", method!(YPositionTryFallbacksPositionArea::second, 0))?;

    let quotes_class = declarations_module.define_class("Quotes", ruby.class_object())?;
    quotes_class.define_method("value", method!(YQuotes::value, 0))?;

    let _quotes_auto_class = quotes_class.define_class("Auto", ruby.class_object())?;

    let _quotes_none_class = quotes_class.define_class("None", ruby.class_object())?;

    let quotes_quote_list_class = quotes_class.define_class("QuoteList", ruby.class_object())?;
    quotes_quote_list_class.define_method("values", method!(YQuotesQuoteList::values, 0))?;

    let quotes_quote_pair_class = quotes_class.define_class("QuotePair", ruby.class_object())?;
    quotes_quote_pair_class.define_method("opening", method!(YQuotesQuotePair::opening, 0))?;
    quotes_quote_pair_class.define_method("closing", method!(YQuotesQuotePair::closing, 0))?;

    let resolution_class = declarations_module.define_class("Resolution", ruby.class_object())?;
    resolution_class.define_method("dpi", method!(YResolution::dpi, 0))?;

    let text_indent_class = declarations_module.define_class("TextIndent", ruby.class_object())?;
    text_indent_class.define_method("length", method!(YTextIndent::length, 0))?;
    text_indent_class.define_method("hanging?", method!(YTextIndent::hanging, 0))?;
    text_indent_class.define_method("each_line?", method!(YTextIndent::each_line, 0))?;

    let text_shadow_class = declarations_module.define_class("TextShadow", ruby.class_object())?;
    text_shadow_class.define_method("values", method!(YTextShadow::values, 0))?;

    let text_shadow_shadow_class = text_shadow_class.define_class("Shadow", ruby.class_object())?;
    text_shadow_shadow_class.define_method("color", method!(YTextShadowShadow::color, 0))?;
    text_shadow_shadow_class.define_method("horizontal", method!(YTextShadowShadow::horizontal, 0))?;
    text_shadow_shadow_class.define_method("vertical", method!(YTextShadowShadow::vertical, 0))?;
    text_shadow_shadow_class.define_method("blur", method!(YTextShadowShadow::blur, 0))?;

    let transform_class = declarations_module.define_class("Transform", ruby.class_object())?;
    transform_class.define_method("operations", method!(YTransform::operations, 0))?;

    let transform_matrix_class = transform_class.define_class("Matrix", ruby.class_object())?;
    transform_matrix_class.define_method("a", method!(YTransformMatrix::a, 0))?;
    transform_matrix_class.define_method("b", method!(YTransformMatrix::b, 0))?;
    transform_matrix_class.define_method("c", method!(YTransformMatrix::c, 0))?;
    transform_matrix_class.define_method("d", method!(YTransformMatrix::d, 0))?;
    transform_matrix_class.define_method("e", method!(YTransformMatrix::e, 0))?;
    transform_matrix_class.define_method("f", method!(YTransformMatrix::f, 0))?;

    let transform_matrix3d_class = transform_class.define_class("Matrix3D", ruby.class_object())?;
    transform_matrix3d_class.define_method("m11", method!(YTransformMatrix3D::m11, 0))?;
    transform_matrix3d_class.define_method("m12", method!(YTransformMatrix3D::m12, 0))?;
    transform_matrix3d_class.define_method("m13", method!(YTransformMatrix3D::m13, 0))?;
    transform_matrix3d_class.define_method("m14", method!(YTransformMatrix3D::m14, 0))?;
    transform_matrix3d_class.define_method("m21", method!(YTransformMatrix3D::m21, 0))?;
    transform_matrix3d_class.define_method("m22", method!(YTransformMatrix3D::m22, 0))?;
    transform_matrix3d_class.define_method("m23", method!(YTransformMatrix3D::m23, 0))?;
    transform_matrix3d_class.define_method("m24", method!(YTransformMatrix3D::m24, 0))?;
    transform_matrix3d_class.define_method("m31", method!(YTransformMatrix3D::m31, 0))?;
    transform_matrix3d_class.define_method("m32", method!(YTransformMatrix3D::m32, 0))?;
    transform_matrix3d_class.define_method("m33", method!(YTransformMatrix3D::m33, 0))?;
    transform_matrix3d_class.define_method("m34", method!(YTransformMatrix3D::m34, 0))?;
    transform_matrix3d_class.define_method("m41", method!(YTransformMatrix3D::m41, 0))?;
    transform_matrix3d_class.define_method("m42", method!(YTransformMatrix3D::m42, 0))?;
    transform_matrix3d_class.define_method("m43", method!(YTransformMatrix3D::m43, 0))?;
    transform_matrix3d_class.define_method("m44", method!(YTransformMatrix3D::m44, 0))?;

    let transform_skew_class = transform_class.define_class("Skew", ruby.class_object())?;
    transform_skew_class.define_method("x", method!(YTransformSkew::x, 0))?;
    transform_skew_class.define_method("y", method!(YTransformSkew::y, 0))?;

    let transform_skew_x_class = transform_class.define_class("SkewX", ruby.class_object())?;
    transform_skew_x_class.define_method("angle", method!(YTransformSkewX::angle, 0))?;

    let transform_skew_y_class = transform_class.define_class("SkewY", ruby.class_object())?;
    transform_skew_y_class.define_method("angle", method!(YTransformSkewY::angle, 0))?;

    let transform_translate_class = transform_class.define_class("Translate", ruby.class_object())?;
    transform_translate_class.define_method("x", method!(YTransformTranslate::x, 0))?;
    transform_translate_class.define_method("y", method!(YTransformTranslate::y, 0))?;

    let transform_translate_x_class = transform_class.define_class("TranslateX", ruby.class_object())?;
    transform_translate_x_class.define_method("value", method!(YTransformTranslateX::value, 0))?;

    let transform_translate_y_class = transform_class.define_class("TranslateY", ruby.class_object())?;
    transform_translate_y_class.define_method("value", method!(YTransformTranslateY::value, 0))?;

    let transform_translate_z_class = transform_class.define_class("TranslateZ", ruby.class_object())?;
    transform_translate_z_class.define_method("value", method!(YTransformTranslateZ::value, 0))?;

    let transform_translate3d_class = transform_class.define_class("Translate3D", ruby.class_object())?;
    transform_translate3d_class.define_method("x", method!(YTransformTranslate3D::x, 0))?;
    transform_translate3d_class.define_method("y", method!(YTransformTranslate3D::y, 0))?;
    transform_translate3d_class.define_method("z", method!(YTransformTranslate3D::z, 0))?;

    let transform_scale_class = transform_class.define_class("Scale", ruby.class_object())?;
    transform_scale_class.define_method("x", method!(YTransformScale::x, 0))?;
    transform_scale_class.define_method("y", method!(YTransformScale::y, 0))?;

    let transform_scale_x_class = transform_class.define_class("ScaleX", ruby.class_object())?;
    transform_scale_x_class.define_method("value", method!(YTransformScaleX::value, 0))?;

    let transform_scale_y_class = transform_class.define_class("ScaleY", ruby.class_object())?;
    transform_scale_y_class.define_method("value", method!(YTransformScaleY::value, 0))?;

    let transform_scale_z_class = transform_class.define_class("ScaleZ", ruby.class_object())?;
    transform_scale_z_class.define_method("value", method!(YTransformScaleZ::value, 0))?;

    let transform_scale3d_class = transform_class.define_class("Scale3D", ruby.class_object())?;
    transform_scale3d_class.define_method("x", method!(YTransformScale3D::x, 0))?;
    transform_scale3d_class.define_method("y", method!(YTransformScale3D::y, 0))?;
    transform_scale3d_class.define_method("z", method!(YTransformScale3D::z, 0))?;

    let transform_rotate_class = transform_class.define_class("Rotate", ruby.class_object())?;
    transform_rotate_class.define_method("angle", method!(YTransformRotate::angle, 0))?;

    let transform_rotate_x_class = transform_class.define_class("RotateX", ruby.class_object())?;
    transform_rotate_x_class.define_method("angle", method!(YTransformRotateX::angle, 0))?;

    let transform_rotate_y_class = transform_class.define_class("RotateY", ruby.class_object())?;
    transform_rotate_y_class.define_method("angle", method!(YTransformRotateY::angle, 0))?;

    let transform_rotate_z_class = transform_class.define_class("RotateZ", ruby.class_object())?;
    transform_rotate_z_class.define_method("angle", method!(YTransformRotateZ::angle, 0))?;

    let transform_rotate3d_class = transform_class.define_class("Rotate3D", ruby.class_object())?;
    transform_rotate3d_class.define_method("x", method!(YTransformRotate3D::x, 0))?;
    transform_rotate3d_class.define_method("y", method!(YTransformRotate3D::y, 0))?;
    transform_rotate3d_class.define_method("z", method!(YTransformRotate3D::z, 0))?;
    transform_rotate3d_class.define_method("angle", method!(YTransformRotate3D::angle, 0))?;

    let transform_perspective_class = transform_class.define_class("Perspective", ruby.class_object())?;
    transform_perspective_class.define_method("value", method!(YTransformPerspective::value, 0))?;

    let _transform_perspective_none_class = transform_perspective_class.define_class("None", ruby.class_object())?;

    let transform_perspective_length_class = transform_perspective_class.define_class("Length", ruby.class_object())?;
    transform_perspective_length_class.define_method("value", method!(YTransformPerspectiveLength::value, 0))?;

    let transform_interpolate_matrix_class = transform_class.define_class("InterpolateMatrix", ruby.class_object())?;
    transform_interpolate_matrix_class.define_method("from_list", method!(YTransformInterpolateMatrix::from_list, 0))?;
    transform_interpolate_matrix_class.define_method("to_list", method!(YTransformInterpolateMatrix::to_list, 0))?;
    transform_interpolate_matrix_class.define_method("progress", method!(YTransformInterpolateMatrix::progress, 0))?;

    let transform_accumulate_matrix_class = transform_class.define_class("AccumulateMatrix", ruby.class_object())?;
    transform_accumulate_matrix_class.define_method("from_list", method!(YTransformAccumulateMatrix::from_list, 0))?;
    transform_accumulate_matrix_class.define_method("to_list", method!(YTransformAccumulateMatrix::to_list, 0))?;
    transform_accumulate_matrix_class.define_method("count", method!(YTransformAccumulateMatrix::count, 0))?;

    let transition_behavior_class = declarations_module.define_class("TransitionBehavior", ruby.class_object())?;
    transition_behavior_class.define_method("values", method!(YTransitionBehavior::values, 0))?;

    let transition_delay_class = declarations_module.define_class("TransitionDelay", ruby.class_object())?;
    transition_delay_class.define_method("values", method!(YTransitionDelay::values, 0))?;

    let transition_duration_class = declarations_module.define_class("TransitionDuration", ruby.class_object())?;
    transition_duration_class.define_method("values", method!(YTransitionDuration::values, 0))?;

    let transition_property_class = declarations_module.define_class("TransitionProperty", ruby.class_object())?;
    transition_property_class.define_method("values", method!(YTransitionProperty::values, 0))?;

    let transition_property_non_custom_class = transition_property_class.define_class("NonCustom", ruby.class_object())?;
    transition_property_non_custom_class.define_method("name", method!(YTransitionPropertyNonCustom::name, 0))?;
    transition_property_non_custom_class.define_method("all?", method!(YTransitionPropertyNonCustom::is_all, 0))?;

    let transition_property_custom_class = transition_property_class.define_class("Custom", ruby.class_object())?;
    transition_property_custom_class.define_method("name", method!(YTransitionPropertyCustom::name, 0))?;

    let transition_property_unsupported_class = transition_property_class.define_class("Unsupported", ruby.class_object())?;
    transition_property_unsupported_class.define_method("name", method!(YTransitionPropertyUnsupported::name, 0))?;
    transition_property_unsupported_class.define_method("none?", method!(YTransitionPropertyUnsupported::is_none, 0))?;

    let transition_timing_function_class = declarations_module.define_class("TransitionTimingFunction", ruby.class_object())?;
    transition_timing_function_class.define_method("values", method!(YTransitionTimingFunction::values, 0))?;

    let translate_class = declarations_module.define_class("Translate", ruby.class_object())?;
    translate_class.define_method("value", method!(YTranslate::value, 0))?;

    let _translate_none_class = translate_class.define_class("None", ruby.class_object())?;

    let translate_coords_class = translate_class.define_class("Coords", ruby.class_object())?;
    translate_coords_class.define_method("x", method!(YTranslateCoords::x, 0))?;
    translate_coords_class.define_method("y", method!(YTranslateCoords::y, 0))?;
    translate_coords_class.define_method("z", method!(YTranslateCoords::z, 0))?;

    let view_transition_class_class = declarations_module.define_class("ViewTransitionClass", ruby.class_object())?;
    view_transition_class_class.define_method("none?", method!(YViewTransitionClass::is_none, 0))?;
    view_transition_class_class.define_method("values", method!(YViewTransitionClass::values, 0))?;

    let view_transition_name_class = declarations_module.define_class("ViewTransitionName", ruby.class_object())?;
    view_transition_name_class.define_method("none?", method!(YViewTransitionName::is_none, 0))?;
    view_transition_name_class.define_method("match_element?", method!(YViewTransitionName::is_match_element, 0))?;
    view_transition_name_class.define_method("name", method!(YViewTransitionName::name, 0))?;

    let will_change_class = declarations_module.define_class("WillChange", ruby.class_object())?;
    will_change_class.define_method("auto?", method!(YWillChange::is_auto, 0))?;
    will_change_class.define_method("values", method!(YWillChange::values, 0))?;
    will_change_class.define_method("stacking_context_unconditional?", method!(YWillChange::is_stacking_context_unconditional, 0))?;
    will_change_class.define_method("transform?", method!(YWillChange::is_transform, 0))?;
    will_change_class.define_method("scroll?", method!(YWillChange::is_scroll, 0))?;
    will_change_class.define_method("contain?", method!(YWillChange::is_contain, 0))?;
    will_change_class.define_method("opacity?", method!(YWillChange::is_opacity, 0))?;
    will_change_class.define_method("perspective?", method!(YWillChange::is_perspective, 0))?;
    will_change_class.define_method("z_index?", method!(YWillChange::is_z_index, 0))?;
    will_change_class.define_method("fixpos_cb_non_svg?", method!(YWillChange::is_fixpos_cb_non_svg, 0))?;
    will_change_class.define_method("position?", method!(YWillChange::is_position, 0))?;
    will_change_class.define_method("view_transition_name?", method!(YWillChange::is_view_transition_name, 0))?;
    will_change_class.define_method("backdrop_root?", method!(YWillChange::is_backdrop_root, 0))?;

    let word_spacing_class = declarations_module.define_class("WordSpacing", ruby.class_object())?;
    word_spacing_class.define_method("value", method!(YWordSpacing::value, 0))?;

    let _word_spacing_normal_class = word_spacing_class.define_class("Normal", ruby.class_object())?;

    let xlang_class = declarations_module.define_class("XLang", ruby.class_object())?;

    let track_size_class = declarations_module.define_class("TrackSize", ruby.class_object())?;

    let track_size_minmax_class = track_size_class.define_class("Minmax", ruby.class_object())?;
    track_size_minmax_class.define_method("min", method!(YTrackSizeMinmax::min, 0))?;
    track_size_minmax_class.define_method("max", method!(YTrackSizeMinmax::max, 0))?;

    let track_size_fit_content_class = track_size_class.define_class("FitContent", ruby.class_object())?;
    track_size_fit_content_class.define_method("value", method!(YTrackSizeFitContent::value, 0))?;

    let track_breadth_class = declarations_module.define_class("TrackBreadth", ruby.class_object())?;
    let track_breadth_length_percentage_class = track_breadth_class.define_class("LengthPercentage", ruby.class_object())?;
    track_breadth_length_percentage_class.define_method("value", method!(YTrackBreadthLengthPercentage::value, 0))?;

    let track_breadth_fr_class = track_breadth_class.define_class("Fr", ruby.class_object())?;
    track_breadth_fr_class.define_method("value", method!(YTrackBreadthFr::value, 0))?;

    let _track_breadth_auto_class = track_breadth_class.define_class("Auto", ruby.class_object())?;
    let _track_breadth_min_content_class = track_breadth_class.define_class("MinContent", ruby.class_object())?;
    let _track_breadth_max_content_class = track_breadth_class.define_class("MaxContent", ruby.class_object())?;

    let grid_template_columns_class = declarations_module.define_class("GridTemplateColumns", ruby.class_object())?;
    grid_template_columns_class.define_method("value", method!(YGridTemplateColumns::value, 0))?;

    let grid_template_module = declarations_module.define_module("GridTemplate")?;

    let _grid_template_none_class = grid_template_module.define_class("None", ruby.class_object())?;
    let _grid_template_masonry_class = grid_template_module.define_class("Masonry", ruby.class_object())?;

    let grid_template_track_list_class = grid_template_module.define_class("TrackList", ruby.class_object())?;
    grid_template_track_list_class.define_method("auto_repeat_index", method!(YGridTemplateTrackList::auto_repeat_index, 0))?;
    grid_template_track_list_class.define_method("explicit?", method!(YGridTemplateTrackList::explicit, 0))?;
    grid_template_track_list_class.define_method("auto_repeat?", method!(YGridTemplateTrackList::auto_repeat, 0))?;
    grid_template_track_list_class.define_method("values", method!(YGridTemplateTrackList::values, 0))?;
    grid_template_track_list_class.define_method("line_names", method!(YGridTemplateTrackList::line_names, 0))?;

    let grid_template_track_list_value_class = grid_template_module.define_class("TrackListValue", ruby.class_object())?;
    grid_template_track_list_value_class.define_method("value", method!(YGridTemplateTrackListValue::value, 0))?;

    let grid_template_track_list_value_track_repeat_class = grid_template_track_list_value_class.define_class("TrackRepeat", ruby.class_object())?;
    grid_template_track_list_value_track_repeat_class.define_method("count", method!(YGridTemplateTrackListValueTrackRepeat::count, 0))?;
    grid_template_track_list_value_track_repeat_class.define_method("line_names", method!(YGridTemplateTrackListValueTrackRepeat::line_names, 0))?;
    grid_template_track_list_value_track_repeat_class.define_method("track_sizes", method!(YGridTemplateTrackListValueTrackRepeat::track_sizes, 0))?;

    let grid_template_repeat_count_module = grid_template_module.define_module("RepeatCount")?;
    let grid_template_repeat_count_number_class = grid_template_repeat_count_module.define_class("Number", ruby.class_object())?;
    grid_template_repeat_count_number_class.define_method("value", method!(YGridTemplateRepeatCountNumber::value, 0))?;

    let _grid_template_repeat_count_auto_fill_class = grid_template_repeat_count_module.define_class("AutoFill", ruby.class_object())?;
    let _grid_template_repeat_count_auto_fit_class = grid_template_repeat_count_module.define_class("AutoFit", ruby.class_object())?;

    let grid_template_subgrid_class = grid_template_module.define_class("Subgrid", ruby.class_object())?;
    grid_template_subgrid_class.define_method("expanded_line_names_length", method!(YGridTemplateSubgrid::expanded_line_names_length, 0))?;
    grid_template_subgrid_class.define_method("line_names", method!(YGridTemplateSubgrid::line_names, 0))?;

    let grid_template_line_name_list_value_class = grid_template_module.define_class("LineNameListValue", ruby.class_object())?;
    grid_template_line_name_list_value_class.define_method("value", method!(YGridTemplateLineNameListValue::value, 0))?;

    let grid_template_line_names_class = grid_template_module.define_class("LineNames", ruby.class_object())?;
    grid_template_line_names_class.define_method("names", method!(YGridTemplateLineNames::names, 0))?;

    let grid_template_line_name_repeat_class = grid_template_module.define_class("LineNameRepeat", ruby.class_object())?;
    grid_template_line_name_repeat_class.define_method("count", method!(YGridTemplateLineNameRepeat::count, 0))?;
    grid_template_line_name_repeat_class.define_method("line_names", method!(YGridTemplateLineNameRepeat::line_names, 0))?;

    let grid_template_rows_class = declarations_module.define_class("GridTemplateRows", ruby.class_object())?;
    grid_template_rows_class.define_method("value", method!(YGridTemplateRows::value, 0))?;

    let border_image_source_class = declarations_module.define_class("BorderImageSource", ruby.class_object())?;
    border_image_source_class.define_method("image", method!(YBorderImageSource::image, 0))?;

    let list_style_image_class = declarations_module.define_class("ListStyleImage", ruby.class_object())?;
    list_style_image_class.define_method("image", method!(YListStyleImage::image, 0))?;

    let grid_auto_columns_class = declarations_module.define_class("GridAutoColumns", ruby.class_object())?;
    grid_auto_columns_class.define_method("values", method!(YGridAutoColumns::values, 0))?;

    let grid_auto_rows_class = declarations_module.define_class("GridAutoRows", ruby.class_object())?;
    grid_auto_rows_class.define_method("values", method!(YGridAutoRows::values, 0))?;

    let column_gap_class = declarations_module.define_class("ColumnGap", ruby.class_object())?;
    column_gap_class.define_method("value", method!(YColumnGap::value, 0))?;
    let _column_gap_normal_class = column_gap_class.define_class("Normal", ruby.class_object())?;
    let column_gap_length_percentage_class = column_gap_class.define_class("LengthPercentage", ruby.class_object())?;
    column_gap_length_percentage_class.define_method("value", method!(YColumnGapLengthPercentage::value, 0))?;

    let row_gap_class = declarations_module.define_class("RowGap", ruby.class_object())?;
    row_gap_class.define_method("value", method!(YRowGap::value, 0))?;

    let _row_gap_normal_class = row_gap_class.define_class("Normal", ruby.class_object())?;

    let row_gap_length_percentage_class = row_gap_class.define_class("LengthPercentage", ruby.class_object())?;
    row_gap_length_percentage_class.define_method("value", method!(YRowGapLengthPercentage::value, 0))?;

    let scale_class = declarations_module.define_class("Scale", ruby.class_object())?;
    scale_class.define_method("value", method!(YScale::value, 0))?;

    let _scale_none_class = scale_class.define_class("None", ruby.class_object())?;

    let scale_scale_class = scale_class.define_class("Coords", ruby.class_object())?;
    scale_scale_class.define_method("x", method!(YScaleCoords::x, 0))?;
    scale_scale_class.define_method("y", method!(YScaleCoords::y, 0))?;
    scale_scale_class.define_method("z", method!(YScaleCoords::z, 0))?;

    let grid_column_end_class = declarations_module.define_class("GridColumnEnd", ruby.class_object())?;
    grid_column_end_class.define_method("ident", method!(YGridColumnEnd::ident, 0))?;
    grid_column_end_class.define_method("line_num", method!(YGridColumnEnd::line_num, 0))?;
    grid_column_end_class.define_method("span?", method!(YGridColumnEnd::span, 0))?;
    grid_column_end_class.define_method("auto?", method!(YGridColumnEnd::auto, 0))?;
    grid_column_end_class.define_method("ident_only?", method!(YGridColumnEnd::ident_only, 0))?;

    let grid_column_start_class = declarations_module.define_class("GridColumnStart", ruby.class_object())?;
    grid_column_start_class.define_method("ident", method!(YGridColumnStart::ident, 0))?;
    grid_column_start_class.define_method("line_num", method!(YGridColumnStart::line_num, 0))?;
    grid_column_start_class.define_method("span?", method!(YGridColumnStart::span, 0))?;
    grid_column_start_class.define_method("auto?", method!(YGridColumnStart::auto, 0))?;
    grid_column_start_class.define_method("ident_only?", method!(YGridColumnStart::ident_only, 0))?;

    let grid_row_end_class = declarations_module.define_class("GridRowEnd", ruby.class_object())?;
    grid_row_end_class.define_method("ident", method!(YGridRowEnd::ident, 0))?;
    grid_row_end_class.define_method("line_num", method!(YGridRowEnd::line_num, 0))?;
    grid_row_end_class.define_method("span?", method!(YGridRowEnd::span, 0))?;
    grid_row_end_class.define_method("auto?", method!(YGridRowEnd::auto, 0))?;
    grid_row_end_class.define_method("ident_only?", method!(YGridRowEnd::ident_only, 0))?;

    let grid_row_start_class = declarations_module.define_class("GridRowStart", ruby.class_object())?;
    grid_row_start_class.define_method("ident", method!(YGridRowStart::ident, 0))?;
    grid_row_start_class.define_method("line_num", method!(YGridRowStart::line_num, 0))?;
    grid_row_start_class.define_method("span?", method!(YGridRowStart::span, 0))?;
    grid_row_start_class.define_method("auto?", method!(YGridRowStart::auto, 0))?;
    grid_row_start_class.define_method("ident_only?", method!(YGridRowStart::ident_only, 0))?;

    let max_block_size_class = declarations_module.define_class("MaxBlockSize", ruby.class_object())?;
    max_block_size_class.define_method("size", method!(YMaxBlockSize::size, 0))?;

    let max_height_class = declarations_module.define_class("MaxHeight", ruby.class_object())?;
    max_height_class.define_method("size", method!(YMaxHeight::size, 0))?;

    let max_inline_size_class = declarations_module.define_class("MaxInlineSize", ruby.class_object())?;
    max_inline_size_class.define_method("size", method!(YMaxInlineSize::size, 0))?;

    let max_width_class = declarations_module.define_class("MaxWidth", ruby.class_object())?;
    max_width_class.define_method("size", method!(YMaxWidth::size, 0))?;

    let bottom_class = declarations_module.define_class("Bottom", ruby.class_object())?;
    bottom_class.define_method("value", method!(YBottom::value, 0))?;

    let inset_class = declarations_module.define_class("Inset", ruby.class_object())?;
    let _inset_auto_class = inset_class.define_class("Auto", ruby.class_object())?;
    let inset_length_percentage_class = inset_class.define_class("LengthPercentage", ruby.class_object())?;
    inset_length_percentage_class.define_method("value", method!(YInsetLengthPercentage::value, 0))?;
    let inset_anchor_containing_calc_function_class = inset_class.define_class("AnchorContainingCalcFunction", ruby.class_object())?;
    inset_anchor_containing_calc_function_class.define_method("value", method!(YInsetAnchorContainingCalcFunction::value, 0))?;
    let inset_anchor_function_class = inset_class.define_class("AnchorFunction", ruby.class_object())?;
    inset_anchor_function_class.define_method("value", method!(YInsetAnchorFunction::value, 0))?;
    let inset_anchor_size_function_class = inset_class.define_class("AnchorSizeFunction", ruby.class_object())?;
    inset_anchor_size_function_class.define_method("value", method!(YInsetAnchorSizeFunction::value, 0))?;

    let inset_block_end_class = declarations_module.define_class("InsetBlockEnd", ruby.class_object())?;
    inset_block_end_class.define_method("value", method!(YInsetBlockEnd::value, 0))?;

    let inset_block_start_class = declarations_module.define_class("InsetBlockStart", ruby.class_object())?;
    inset_block_start_class.define_method("value", method!(YInsetBlockStart::value, 0))?;

    let inset_inline_end_class = declarations_module.define_class("InsetInlineEnd", ruby.class_object())?;
    inset_inline_end_class.define_method("value", method!(YInsetInlineEnd::value, 0))?;

    let inset_inline_start_class = declarations_module.define_class("InsetInlineStart", ruby.class_object())?;
    inset_inline_start_class.define_method("value", method!(YInsetInlineStart::value, 0))?;

    let left_class = declarations_module.define_class("Left", ruby.class_object())?;
    left_class.define_method("value", method!(YLeft::value, 0))?;

    let length_module = declarations_module.define_module("Length")?;

    let absolute_length_class = length_module.define_class("Absolute", ruby.class_object())?;
    absolute_length_class.define_method("value", method!(YAbsoluteLength::value, 0))?;
    absolute_length_class.define_method("unit", method!(YAbsoluteLength::unit, 0))?;

    let font_relative_length_class = length_module.define_class("FontRelative", ruby.class_object())?;
    font_relative_length_class.define_method("value", method!(YFontRelativeLength::value, 0))?;
    font_relative_length_class.define_method("unit", method!(YFontRelativeLength::unit, 0))?;

    let viewport_percentage_length_class = length_module.define_class("ViewportPercentage", ruby.class_object())?;
    viewport_percentage_length_class.define_method("value", method!(YViewportPercentageLength::value, 0))?;
    viewport_percentage_length_class.define_method("unit", method!(YViewportPercentageLength::unit, 0))?;

    let container_relative_length_class = length_module.define_class("ContainerRelative", ruby.class_object())?;
    container_relative_length_class.define_method("value", method!(YContainerRelativeLength::value, 0))?;
    container_relative_length_class.define_method("unit", method!(YContainerRelativeLength::unit, 0))?;

    let character_width_length_class = length_module.define_class("CharacterWidth", ruby.class_object())?;
    character_width_length_class.define_method("value", method!(YCharacterWidthLength::value, 0))?;

    let right_class = declarations_module.define_class("Right", ruby.class_object())?;
    right_class.define_method("value", method!(YRight::value, 0))?;

    let rotate_class = declarations_module.define_class("Rotate", ruby.class_object())?;
    rotate_class.define_method("value", method!(YRotate::value, 0))?;

    let _rotate_none_class = rotate_class.define_class("None", ruby.class_object())?;

    let rotate_rotate3d_class = rotate_class.define_class("Rotate3D", ruby.class_object())?;
    rotate_rotate3d_class.define_method("x", method!(YRotate3D::x, 0))?;
    rotate_rotate3d_class.define_method("y", method!(YRotate3D::y, 0))?;
    rotate_rotate3d_class.define_method("z", method!(YRotate3D::z, 0))?;
    rotate_rotate3d_class.define_method("angle", method!(YRotate3D::angle, 0))?;

    let top_class = declarations_module.define_class("Top", ruby.class_object())?;
    top_class.define_method("value", method!(YTop::value, 0))?;

    let transform_origin_class = declarations_module.define_class("TransformOrigin", ruby.class_object())?;
    transform_origin_class.define_method("horizontal", method!(YTransformOrigin::horizontal, 0))?;
    transform_origin_class.define_method("vertical", method!(YTransformOrigin::vertical, 0))?;
    transform_origin_class.define_method("depth", method!(YTransformOrigin::depth, 0))?;

    let _transform_origin_center_class = transform_origin_class.define_class("Center", ruby.class_object())?;

    let transform_origin_side_horizontal_class = transform_origin_class.define_class("SideHorizontalComponent", ruby.class_object())?;
    transform_origin_side_horizontal_class.define_method("side", method!(YSideHorizontalOriginComponent::side, 0))?;

    let transform_origin_side_vertical_class = transform_origin_class.define_class("SideVerticalComponent", ruby.class_object())?;
    transform_origin_side_vertical_class.define_method("side", method!(YSideVerticalOriginComponent::side, 0))?;

    let margin_module = declarations_module.define_module("Margin")?;

    let _margin_auto_class = margin_module.define_class("Auto", ruby.class_object())?;

    let margin_anchor_size_function_class = margin_module.define_class("AnchorSizeFunction", ruby.class_object())?;
    margin_anchor_size_function_class.define_method("target_element", method!(YMarginAnchorSizeFunction::target_element, 0))?;
    margin_anchor_size_function_class.define_method("size", method!(YMarginAnchorSizeFunction::size, 0))?;
    margin_anchor_size_function_class.define_method("fallback", method!(YMarginAnchorSizeFunction::fallback, 0))?;

    let margin_anchor_containing_calc_function_class = margin_module.define_class("AnchorContainingCalcFunction", ruby.class_object())?;
    margin_anchor_containing_calc_function_class.define_method("value", method!(YMarginAnchorContainingCalcFunction::value, 0))?;

    let margin_block_end_class = declarations_module.define_class("MarginBlockEnd", ruby.class_object())?;
    margin_block_end_class.define_method("value", method!(YMarginBlockEnd::value, 0))?;

    let margin_block_start_class = declarations_module.define_class("MarginBlockStart", ruby.class_object())?;
    margin_block_start_class.define_method("value", method!(YMarginBlockStart::value, 0))?;

    let margin_bottom_class = declarations_module.define_class("MarginBottom", ruby.class_object())?;
    margin_bottom_class.define_method("value", method!(YMarginBottom::value, 0))?;

    let margin_inline_end_class = declarations_module.define_class("MarginInlineEnd", ruby.class_object())?;
    margin_inline_end_class.define_method("value", method!(YMarginInlineEnd::value, 0))?;

    let margin_inline_start_class = declarations_module.define_class("MarginInlineStart", ruby.class_object())?;
    margin_inline_start_class.define_method("value", method!(YMarginInlineStart::value, 0))?;

    let margin_left_class = declarations_module.define_class("MarginLeft", ruby.class_object())?;
    margin_left_class.define_method("value", method!(YMarginLeft::value, 0))?;

    let margin_right_class = declarations_module.define_class("MarginRight", ruby.class_object())?;
    margin_right_class.define_method("value", method!(YMarginRight::value, 0))?;

    let margin_top_class = declarations_module.define_class("MarginTop", ruby.class_object())?;
    margin_top_class.define_method("value", method!(YMarginTop::value, 0))?;

    let padding_block_end_class = declarations_module.define_class("PaddingBlockEnd", ruby.class_object())?;
    padding_block_end_class.define_method("value", method!(YPaddingBlockEnd::value, 0))?;

    let padding_block_start_class = declarations_module.define_class("PaddingBlockStart", ruby.class_object())?;
    padding_block_start_class.define_method("value", method!(YPaddingBlockStart::value, 0))?;

    let padding_bottom_class = declarations_module.define_class("PaddingBottom", ruby.class_object())?;
    padding_bottom_class.define_method("value", method!(YPaddingBottom::value, 0))?;

    let padding_inline_end_class = declarations_module.define_class("PaddingInlineEnd", ruby.class_object())?;
    padding_inline_end_class.define_method("value", method!(YPaddingInlineEnd::value, 0))?;

    let padding_inline_start_class = declarations_module.define_class("PaddingInlineStart", ruby.class_object())?;
    padding_inline_start_class.define_method("value", method!(YPaddingInlineStart::value, 0))?;

    let padding_left_class = declarations_module.define_class("PaddingLeft", ruby.class_object())?;
    padding_left_class.define_method("value", method!(YPaddingLeft::value, 0))?;

    let padding_right_class = declarations_module.define_class("PaddingRight", ruby.class_object())?;
    padding_right_class.define_method("value", method!(YPaddingRight::value, 0))?;

    let padding_top_class = declarations_module.define_class("PaddingTop", ruby.class_object())?;
    padding_top_class.define_method("value", method!(YPaddingTop::value, 0))?;

    let perspective_origin_class = declarations_module.define_class("PerspectiveOrigin", ruby.class_object())?;
    perspective_origin_class.define_method("horizontal", method!(YPerspectiveOrigin::horizontal, 0))?;
    perspective_origin_class.define_method("vertical", method!(YPerspectiveOrigin::vertical, 0))?;

    let block_size_class = declarations_module.define_class("BlockSize", ruby.class_object())?;
    block_size_class.define_method("size", method!(YBlockSize::size, 0))?;

    let height_class = declarations_module.define_class("Height", ruby.class_object())?;
    height_class.define_method("size", method!(YHeight::size, 0))?;

    let inline_size_class = declarations_module.define_class("InlineSize", ruby.class_object())?;
    inline_size_class.define_method("size", method!(YInlineSize::size, 0))?;

    let min_block_size_class = declarations_module.define_class("MinBlockSize", ruby.class_object())?;
    min_block_size_class.define_method("size", method!(YMinBlockSize::size, 0))?;

    let min_height_class = declarations_module.define_class("MinHeight", ruby.class_object())?;
    min_height_class.define_method("size", method!(YMinHeight::size, 0))?;

    let min_inline_size_class = declarations_module.define_class("MinInlineSize", ruby.class_object())?;
    min_inline_size_class.define_method("size", method!(YMinInlineSize::size, 0))?;

    let min_width_class = declarations_module.define_class("MinWidth", ruby.class_object())?;
    min_width_class.define_method("size", method!(YMinWidth::size, 0))?;

    let width_class = declarations_module.define_class("Width", ruby.class_object())?;
    width_class.define_method("size", method!(YWidth::size, 0))?;

    let border_block_end_width_class = declarations_module.define_class("BorderBlockEndWidth", ruby.class_object())?;
    border_block_end_width_class.define_method("value", method!(YBorderBlockEndWidth::value, 0))?;

    let border_block_start_width_class = declarations_module.define_class("BorderBlockStartWidth", ruby.class_object())?;
    border_block_start_width_class.define_method("value", method!(YBorderBlockStartWidth::value, 0))?;

    let border_bottom_width_class = declarations_module.define_class("BorderBottomWidth", ruby.class_object())?;
    border_bottom_width_class.define_method("value", method!(YBorderBottomWidth::value, 0))?;

    let border_inline_end_width_class = declarations_module.define_class("BorderInlineEndWidth", ruby.class_object())?;
    border_inline_end_width_class.define_method("value", method!(YBorderInlineEndWidth::value, 0))?;

    let border_inline_start_width_class = declarations_module.define_class("BorderInlineStartWidth", ruby.class_object())?;
    border_inline_start_width_class.define_method("value", method!(YBorderInlineStartWidth::value, 0))?;

    let border_left_width_class = declarations_module.define_class("BorderLeftWidth", ruby.class_object())?;
    border_left_width_class.define_method("value", method!(YBorderLeftWidth::value, 0))?;

    let border_right_width_class = declarations_module.define_class("BorderRightWidth", ruby.class_object())?;
    border_right_width_class.define_method("value", method!(YBorderRightWidth::value, 0))?;

    let border_top_width_class = declarations_module.define_class("BorderTopWidth", ruby.class_object())?;
    border_top_width_class.define_method("value", method!(YBorderTopWidth::value, 0))?;

    let outline_width_class = declarations_module.define_class("OutlineWidth", ruby.class_object())?;
    outline_width_class.define_method("value", method!(YOutlineWidth::value, 0))?;

    let background_color_class = declarations_module.define_class("BackgroundColor", ruby.class_object())?;
    background_color_class.define_method("color", method!(YBackgroundColor::color, 0))?;

    let border_block_end_color_class = declarations_module.define_class("BorderBlockEndColor", ruby.class_object())?;
    border_block_end_color_class.define_method("color", method!(YBorderBlockEndColor::color, 0))?;

    let border_block_start_color_class = declarations_module.define_class("BorderBlockStartColor", ruby.class_object())?;
    border_block_start_color_class.define_method("color", method!(YBorderBlockStartColor::color, 0))?;

    let border_bottom_color_class = declarations_module.define_class("BorderBottomColor", ruby.class_object())?;
    border_bottom_color_class.define_method("color", method!(YBorderBottomColor::color, 0))?;

    let border_inline_end_color_class = declarations_module.define_class("BorderInlineEndColor", ruby.class_object())?;
    border_inline_end_color_class.define_method("color", method!(YBorderInlineEndColor::color, 0))?;

    let border_inline_start_color_class = declarations_module.define_class("BorderInlineStartColor", ruby.class_object())?;
    border_inline_start_color_class.define_method("color", method!(YBorderInlineStartColor::color, 0))?;

    let border_left_color_class = declarations_module.define_class("BorderLeftColor", ruby.class_object())?;
    border_left_color_class.define_method("color", method!(YBorderLeftColor::color, 0))?;

    let border_right_color_class = declarations_module.define_class("BorderRightColor", ruby.class_object())?;
    border_right_color_class.define_method("color", method!(YBorderRightColor::color, 0))?;

    let border_top_color_class = declarations_module.define_class("BorderTopColor", ruby.class_object())?;
    border_top_color_class.define_method("color", method!(YBorderTopColor::color, 0))?;

    let outline_color_class = declarations_module.define_class("OutlineColor", ruby.class_object())?;
    outline_color_class.define_method("color", method!(YOutlineColor::color, 0))?;

    let text_decoration_color_class = declarations_module.define_class("TextDecorationColor", ruby.class_object())?;
    text_decoration_color_class.define_method("color", method!(YTextDecorationColor::color, 0))?;

    let csswide_keyword_class = declarations_module.define_class("CSSWideKeyword", ruby.class_object())?;
    csswide_keyword_class.define_method("value", method!(YCSSWideKeyword::value, 0))?;

    let with_variables_class = declarations_module.define_class("WithVariables", ruby.class_object())?;
    with_variables_class.define_method("value", method!(YWithVariables::value, 0))?;

    let custom_class = declarations_module.define_class("Custom", ruby.class_object())?;
    custom_class.define_method("name", method!(YCustom::name, 0))?;
    custom_class.define_method("value", method!(YCustom::value, 0))?;

    let custom_unparsed_class = custom_class.define_class("Unparsed", ruby.class_object())?;
    custom_unparsed_class.define_method("value", method!(YCustomValueUnparsed::value, 0))?;

    let custom_parsed_class = custom_class.define_class("Parsed", ruby.class_object())?;
    custom_parsed_class.define_method("value", method!(YCustomValueParsed::value, 0))?;

    let custom_csswide_keyword_class = custom_class.define_class("CSSWideKeyword", ruby.class_object())?;
    custom_csswide_keyword_class.define_method("value", method!(YCustomValueCSSWideKeyword::value, 0))?;

    // helper classes
    let time_class = declarations_module.define_class("Time", ruby.class_object())?;
    time_class.define_method("seconds", method!(YTime::seconds, 0))?;
    time_class.define_method("unit", method!(YTime::unit, 0))?;

    let animation_duration_value_class = declarations_module.define_class("AnimationDurationValue", ruby.class_object())?;
    animation_duration_value_class.define_method("time", method!(YAnimationDurationValue::time, 0))?;

    let percentage_class = declarations_module.define_class("Percentage", ruby.class_object())?;
    percentage_class.define_method("value", method!(YPercentage::value, 0))?;

    let number_class = declarations_module.define_class("Number", ruby.class_object())?;
    number_class.define_method("value", method!(YNumber::value, 0))?;

    animation::init::init(ruby, yass_module, &declarations_module)?;
    calc::init::init(ruby, yass_module, &declarations_module)?;
    clip_path::init::init(ruby, &declarations_module)?;
    color::init::init(ruby, &color_class)?;
    filter::init::init(ruby, yass_module, &declarations_module)?;
    images::init::init(ruby, &declarations_module)?;
    size::init::init(ruby, yass_module, &declarations_module)?;

    Ok(())
}
