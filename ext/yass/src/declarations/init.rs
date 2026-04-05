use magnus::{Error, Module, RModule, Ruby, method};

use crate::declarations::{align_content::YAlignContent, align_items::YAlignItems, align_self::YAlignSelf, alignment_baseline::YAlignmentBaseline, angle::YAngle, animation, animation_composition::YAnimationComposition, animation_delay::YAnimationDelay, animation_direction::YAnimationDirection, animation_duration::{YAnimationDuration, YAnimationDurationValue}, animation_fill_mode::YAnimationFillMode, animation_iteration_count::YAnimationIterationCount, animation_name::YAnimationName, animation_play_state::YAnimationPlayState, animation_range_end::YAnimationRangeEnd, animation_range_start::YAnimationRangeStart, animation_timeline::YAnimationTimeline, animation_timing_function::YAnimationTimingFunction, aspect_ratio::YAspectRatio, backdrop_filter::YBackdropFilter, backface_visibility::YBackfaceVisibility, background_attachment::YBackgroundAttachment, background_blend_mode::YBackgroundBlendMode, background_clip::YBackgroundClip, background_color::YBackgroundColor, background_image::YBackgroundImage, background_origin::YBackgroundOrigin, background_position_x::YBackgroundPositionX, background_position_y::YBackgroundPositionY, background_repeat::{YBackgroundRepeat, YBackgroundRepeatValue}, background_size::{YBackgroundSize, YBackgroundSizeExplicitSize}, baseline_shift::{YBaselineShiftKeyword, YBaselineShiftLength}, baseline_source::YBaselineSource, block_size::YBlockSize, border_block_end_color::YBorderBlockEndColor, border_block_end_style::YBorderBlockEndStyle, border_block_end_width::YBorderBlockEndWidth, border_block_start_color::YBorderBlockStartColor, border_block_start_style::YBorderBlockStartStyle, border_block_start_width::YBorderBlockStartWidth, border_bottom_color::YBorderBottomColor, border_bottom_left_radius::YBorderBottomLeftRadius, border_bottom_right_radius::YBorderBottomRightRadius, border_bottom_style::YBorderBottomStyle, border_bottom_width::YBorderBottomWidth, border_collapse::YBorderCollapse, border_end_end_radius::YBorderEndEndRadius, border_end_start_radius::YBorderEndStartRadius, border_image_outset::YBorderImageOutset, border_image_repeat::YBorderImageRepeat, border_image_slice::YBorderImageSlice, border_image_source::YBorderImageSource, border_image_width::YBorderImageWidth, border_inline_end_color::YBorderInlineEndColor, border_inline_end_style::YBorderInlineEndStyle, border_inline_end_width::YBorderInlineEndWidth, border_inline_start_color::YBorderInlineStartColor, border_inline_start_style::YBorderInlineStartStyle, border_inline_start_width::YBorderInlineStartWidth, border_left_color::YBorderLeftColor, border_left_style::YBorderLeftStyle, border_left_width::YBorderLeftWidth, border_right_color::YBorderRightColor, border_right_style::YBorderRightStyle, border_right_width::YBorderRightWidth, border_spacing::YBorderSpacing, border_start_end_radius::YBorderStartEndRadius, border_start_start_radius::YBorderStartStartRadius, border_top_color::YBorderTopColor, border_top_left_radius::YBorderTopLeftRadius, border_top_right_radius::YBorderTopRightRadius, border_top_style::YBorderTopStyle, border_top_width::YBorderTopWidth, bottom::YBottom, box_shadow::{YBoxShadow, YBoxShadowShadow}, box_sizing::YBoxSizing, calc, caption_side::YCaptionSide, caret_color::{self, YCaretColor}, channel_keyword::YChannelKeyword, clear::YClear, clip::{YClip, YClipLength, YClipRect}, clip_path, color::{self, YColor}, color_scheme::YColorScheme, column_count::YColumnCountInteger, column_gap::{YColumnGap, YColumnGapLengthPercentage}, column_span::YColumnSpan, column_width::{YColumnWidth, YColumnWidthLength}, contain::YContain, container_name::YContainerName, container_type::YContainerType, content::{YContent, YContentCounterStyleSymbols, YContentItemAttr, YContentItemCounter, YContentItemCounters, YContentItemImage, YContentItemString, YContentItems, YContentNone, YContentNormal}, counter_increment::{YCounterIncrement, YCounterIncrementCounter}, counter_reset::{YCounterReset, YCounterResetCounter}, csswide_keyword::YCSSWideKeyword, cursor::{YCursor, YCursorImage}, custom::{YCustom, YCustomValueCSSWideKeyword, YCustomValueParsed, YCustomValueUnparsed}, direction::YDirection, display::YDisplay, empty_cells::YEmptyCells, filter, flex_basis::{YFlexBasis, YFlexBasisSize}, flex_direction::YFlexDirection, flex_grow::YFlexGrow, flex_shrink::YFlexShrink, flex_wrap::YFlexWrap, images, inset::{YInsetAnchorContainingCalcFunction, YInsetAnchorFunction, YInsetAnchorSizeFunction, YInsetLengthPercentage}, left::YLeft, length::{YAbsoluteLength, YCharacterWidthLength, YContainerRelativeLength, YFontRelativeLength, YViewportPercentageLength}, list_style_image::YListStyleImage, mask_image::YMaskImage, number::YNumber, outline_color::YOutlineColor, percentage::YPercentage, resolution::YResolution, right::YRight, size, text_decoration_color::YTextDecorationColor, time::YTime, top::YTop, width::YWidth};

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

    let font_language_override_class = declarations_module.define_class("FontLanguageOverride", ruby.class_object())?;

    let font_optical_sizing_class = declarations_module.define_class("FontOpticalSizing", ruby.class_object())?;

    let font_stretch_class = declarations_module.define_class("FontStretch", ruby.class_object())?;

    let font_style_class = declarations_module.define_class("FontStyle", ruby.class_object())?;

    let font_synthesis_weight_class = declarations_module.define_class("FontSynthesisWeight", ruby.class_object())?;

    let font_variant_caps_class = declarations_module.define_class("FontVariantCaps", ruby.class_object())?;

    let font_weight_class = declarations_module.define_class("FontWeight", ruby.class_object())?;

    let grid_auto_flow_class = declarations_module.define_class("GridAutoFlow", ruby.class_object())?;

    let image_rendering_class = declarations_module.define_class("ImageRendering", ruby.class_object())?;

    let isolation_class = declarations_module.define_class("Isolation", ruby.class_object())?;

    let justify_items_class = declarations_module.define_class("JustifyItems", ruby.class_object())?;

    let line_break_class = declarations_module.define_class("LineBreak", ruby.class_object())?;

    let list_style_position_class = declarations_module.define_class("ListStylePosition", ruby.class_object())?;

    let mix_blend_mode_class = declarations_module.define_class("MixBlendMode", ruby.class_object())?;

    let object_fit_class = declarations_module.define_class("ObjectFit", ruby.class_object())?;

    let object_position_class = declarations_module.define_class("ObjectPosition", ruby.class_object())?;

    let opacity_class = declarations_module.define_class("Opacity", ruby.class_object())?;

    let order_class = declarations_module.define_class("Order", ruby.class_object())?;

    let outline_style_class = declarations_module.define_class("OutlineStyle", ruby.class_object())?;

    let overflow_wrap_class = declarations_module.define_class("OverflowWrap", ruby.class_object())?;

    let pointer_events_class = declarations_module.define_class("PointerEvents", ruby.class_object())?;

    let position_class = declarations_module.define_class("Position", ruby.class_object())?;

    let position_area_class = declarations_module.define_class("PositionArea", ruby.class_object())?;

    let servo_overflow_clip_box_class = declarations_module.define_class("ServoOverflowClipBox", ruby.class_object())?;

    let servo_top_layer_class = declarations_module.define_class("ServoTopLayer", ruby.class_object())?;

    let table_layout_class = declarations_module.define_class("TableLayout", ruby.class_object())?;

    let text_align_class = declarations_module.define_class("TextAlign", ruby.class_object())?;

    let text_align_last_class = declarations_module.define_class("TextAlignLast", ruby.class_object())?;

    let text_decoration_line_class = declarations_module.define_class("TextDecorationLine", ruby.class_object())?;

    let text_decoration_style_class = declarations_module.define_class("TextDecorationStyle", ruby.class_object())?;

    let text_overflow_class = declarations_module.define_class("TextOverflow", ruby.class_object())?;

    let text_justify_class = declarations_module.define_class("TextJustify", ruby.class_object())?;

    let text_rendering_class = declarations_module.define_class("TextRendering", ruby.class_object())?;

    let text_transform_class = declarations_module.define_class("TextTransform", ruby.class_object())?;

    let text_wrap_mode_class = declarations_module.define_class("TextWrapMode", ruby.class_object())?;

    let transform_style_class = declarations_module.define_class("TransformStyle", ruby.class_object())?;

    let unicode_bidi_class = declarations_module.define_class("UnicodeBidi", ruby.class_object())?;

    let visibility_class = declarations_module.define_class("Visibility", ruby.class_object())?;

    let webkit_text_security_class = declarations_module.define_class("WebkitTextSecurity", ruby.class_object())?;

    let white_space_collapse_class = declarations_module.define_class("WhiteSpaceCollapse", ruby.class_object())?;

    let word_break_class = declarations_module.define_class("WordBreak", ruby.class_object())?;

    let writing_mode_class = declarations_module.define_class("WritingMode", ruby.class_object())?;

    let zindex_class = declarations_module.define_class("ZIndex", ruby.class_object())?;

    let zoom_class = declarations_module.define_class("Zoom", ruby.class_object())?;

    let align_content_class = declarations_module.define_class("AlignContent", ruby.class_object())?;
    align_content_class.define_method("value", method!(YAlignContent::value, 0))?;

    let justify_content_class = declarations_module.define_class("JustifyContent", ruby.class_object())?;

    let flex_grow_class = declarations_module.define_class("FlexGrow", ruby.class_object())?;
    flex_grow_class.define_method("value", method!(YFlexGrow::value, 0))?;

    let flex_shrink_class = declarations_module.define_class("FlexShrink", ruby.class_object())?;
    flex_shrink_class.define_method("value", method!(YFlexShrink::value, 0))?;

    let align_self_class = declarations_module.define_class("AlignSelf", ruby.class_object())?;
    align_self_class.define_method("value", method!(YAlignSelf::value, 0))?;

    let justify_self_class = declarations_module.define_class("JustifySelf", ruby.class_object())?;

    let overflow_block_class = declarations_module.define_class("OverflowBlock", ruby.class_object())?;

    let overflow_inline_class = declarations_module.define_class("OverflowInline", ruby.class_object())?;

    let overflow_x_class = declarations_module.define_class("OverflowX", ruby.class_object())?;

    let overflow_y_class = declarations_module.define_class("OverflowY", ruby.class_object())?;

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

    clip_path::init::init(ruby, &declarations_module)?;

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

    let font_size_class = declarations_module.define_class("FontSize", ruby.class_object())?;

    let font_variation_settings_class = declarations_module.define_class("FontVariationSettings", ruby.class_object())?;

    let grid_template_areas_class = declarations_module.define_class("GridTemplateAreas", ruby.class_object())?;

    let letter_spacing_class = declarations_module.define_class("LetterSpacing", ruby.class_object())?;

    let line_height_class = declarations_module.define_class("LineHeight", ruby.class_object())?;

    let list_style_type_class = declarations_module.define_class("ListStyleType", ruby.class_object())?;

    let mask_image_class = declarations_module.define_class("MaskImage", ruby.class_object())?;
    mask_image_class.define_method("values", method!(YMaskImage::values, 0))?;

    let offset_path_class = declarations_module.define_class("OffsetPath", ruby.class_object())?;

    let outline_offset_class = declarations_module.define_class("OutlineOffset", ruby.class_object())?;

    let overflow_clip_margin_class = declarations_module.define_class("OverflowClipMargin", ruby.class_object())?;

    let perspective_class = declarations_module.define_class("Perspective", ruby.class_object())?;

    let position_try_fallbacks_class = declarations_module.define_class("PositionTryFallbacks", ruby.class_object())?;

    let quotes_class = declarations_module.define_class("Quotes", ruby.class_object())?;

    let resolution_class = declarations_module.define_class("Resolution", ruby.class_object())?;
    resolution_class.define_method("dpi", method!(YResolution::dpi, 0))?;

    let text_indent_class = declarations_module.define_class("TextIndent", ruby.class_object())?;

    let text_shadow_class = declarations_module.define_class("TextShadow", ruby.class_object())?;

    let transform_class = declarations_module.define_class("Transform", ruby.class_object())?;

    let transition_behavior_class = declarations_module.define_class("TransitionBehavior", ruby.class_object())?;

    let transition_delay_class = declarations_module.define_class("TransitionDelay", ruby.class_object())?;

    let transition_duration_class = declarations_module.define_class("TransitionDuration", ruby.class_object())?;

    let transition_property_class = declarations_module.define_class("TransitionProperty", ruby.class_object())?;

    let transition_timing_function_class = declarations_module.define_class("TransitionTimingFunction", ruby.class_object())?;

    let translate_class = declarations_module.define_class("Translate", ruby.class_object())?;

    let view_transition_class_class = declarations_module.define_class("ViewTransitionClass", ruby.class_object())?;

    let view_transition_name_class = declarations_module.define_class("ViewTransitionName", ruby.class_object())?;

    let will_change_class = declarations_module.define_class("WillChange", ruby.class_object())?;

    let word_spacing_class = declarations_module.define_class("WordSpacing", ruby.class_object())?;

    let xlang_class = declarations_module.define_class("XLang", ruby.class_object())?;

    let grid_template_columns_class = declarations_module.define_class("GridTemplateColumns", ruby.class_object())?;

    let grid_template_rows_class = declarations_module.define_class("GridTemplateRows", ruby.class_object())?;

    let border_image_source_class = declarations_module.define_class("BorderImageSource", ruby.class_object())?;
    border_image_source_class.define_method("image", method!(YBorderImageSource::image, 0))?;

    let list_style_image_class = declarations_module.define_class("ListStyleImage", ruby.class_object())?;
    list_style_image_class.define_method("image", method!(YListStyleImage::image, 0))?;

    let grid_auto_columns_class = declarations_module.define_class("GridAutoColumns", ruby.class_object())?;

    let grid_auto_rows_class = declarations_module.define_class("GridAutoRows", ruby.class_object())?;

    let column_gap_class = declarations_module.define_class("ColumnGap", ruby.class_object())?;
    column_gap_class.define_method("value", method!(YColumnGap::value, 0))?;
    let _column_gap_normal_class = column_gap_class.define_class("Normal", ruby.class_object())?;
    let column_gap_length_percentage_class = column_gap_class.define_class("LengthPercentage", ruby.class_object())?;
    column_gap_length_percentage_class.define_method("value", method!(YColumnGapLengthPercentage::value, 0))?;

    let row_gap_class = declarations_module.define_class("RowGap", ruby.class_object())?;

    let scale_class = declarations_module.define_class("Scale", ruby.class_object())?;

    let grid_column_end_class = declarations_module.define_class("GridColumnEnd", ruby.class_object())?;

    let grid_column_start_class = declarations_module.define_class("GridColumnStart", ruby.class_object())?;

    let grid_row_end_class = declarations_module.define_class("GridRowEnd", ruby.class_object())?;

    let grid_row_start_class = declarations_module.define_class("GridRowStart", ruby.class_object())?;

    let max_block_size_class = declarations_module.define_class("MaxBlockSize", ruby.class_object())?;

    let max_height_class = declarations_module.define_class("MaxHeight", ruby.class_object())?;

    let max_inline_size_class = declarations_module.define_class("MaxInlineSize", ruby.class_object())?;

    let max_width_class = declarations_module.define_class("MaxWidth", ruby.class_object())?;

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

    let inset_block_start_class = declarations_module.define_class("InsetBlockStart", ruby.class_object())?;

    let inset_inline_end_class = declarations_module.define_class("InsetInlineEnd", ruby.class_object())?;

    let inset_inline_start_class = declarations_module.define_class("InsetInlineStart", ruby.class_object())?;

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

    let top_class = declarations_module.define_class("Top", ruby.class_object())?;
    top_class.define_method("value", method!(YTop::value, 0))?;

    let transform_origin_class = declarations_module.define_class("TransformOrigin", ruby.class_object())?;

    let margin_block_end_class = declarations_module.define_class("MarginBlockEnd", ruby.class_object())?;

    let margin_block_start_class = declarations_module.define_class("MarginBlockStart", ruby.class_object())?;

    let margin_bottom_class = declarations_module.define_class("MarginBottom", ruby.class_object())?;

    let margin_inline_end_class = declarations_module.define_class("MarginInlineEnd", ruby.class_object())?;

    let margin_inline_start_class = declarations_module.define_class("MarginInlineStart", ruby.class_object())?;

    let margin_left_class = declarations_module.define_class("MarginLeft", ruby.class_object())?;

    let margin_right_class = declarations_module.define_class("MarginRight", ruby.class_object())?;

    let margin_top_class = declarations_module.define_class("MarginTop", ruby.class_object())?;

    let padding_block_end_class = declarations_module.define_class("PaddingBlockEnd", ruby.class_object())?;

    let padding_block_start_class = declarations_module.define_class("PaddingBlockStart", ruby.class_object())?;

    let padding_bottom_class = declarations_module.define_class("PaddingBottom", ruby.class_object())?;

    let padding_inline_end_class = declarations_module.define_class("PaddingInlineEnd", ruby.class_object())?;

    let padding_inline_start_class = declarations_module.define_class("PaddingInlineStart", ruby.class_object())?;

    let padding_left_class = declarations_module.define_class("PaddingLeft", ruby.class_object())?;

    let padding_right_class = declarations_module.define_class("PaddingRight", ruby.class_object())?;

    let padding_top_class = declarations_module.define_class("PaddingTop", ruby.class_object())?;

    let perspective_origin_class = declarations_module.define_class("PerspectiveOrigin", ruby.class_object())?;

    let block_size_class = declarations_module.define_class("BlockSize", ruby.class_object())?;
    block_size_class.define_method("size", method!(YBlockSize::size, 0))?;

    let height_class = declarations_module.define_class("Height", ruby.class_object())?;

    let inline_size_class = declarations_module.define_class("InlineSize", ruby.class_object())?;

    let min_block_size_class = declarations_module.define_class("MinBlockSize", ruby.class_object())?;

    let min_height_class = declarations_module.define_class("MinHeight", ruby.class_object())?;

    let min_inline_size_class = declarations_module.define_class("MinInlineSize", ruby.class_object())?;

    let min_width_class = declarations_module.define_class("MinWidth", ruby.class_object())?;

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
    color::init::init(ruby, &color_class)?;
    filter::init::init(ruby, yass_module, &declarations_module)?;
    images::init::init(ruby, &declarations_module)?;
    size::init::init(ruby, yass_module, &declarations_module)?;

    Ok(())
}
