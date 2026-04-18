# frozen_string_literal: true

module Yass
  class Visitor
    def visit(node)
      node.accept(self) if node
    end

    def visit_list(nodes)
      nodes.each { |node| visit(node) }
    end

    def visit_stylesheet(node)
      visit_list(node.rules)
    end

    def visit_declaration_align_content(node)
    end

    def visit_declaration_align_items(node)
    end

    def visit_declaration_align_self(node)
    end

    def visit_declaration_alignment_baseline(node)
    end

    def visit_declaration_angle(node)
    end

    def visit_declaration_animation_auto(node)
    end

    def visit_declaration_animation_inset(node)
      visit(node.start)
      visit(node.end)
    end

    def visit_declaration_animation_length_auto(node)
    end

    def visit_declaration_animation_range_value(node)
      visit(node.length_percentage)
    end

    def visit_declaration_animation_scroll_axis(node)
    end

    def visit_declaration_animation_scroll_function(node)
      visit(node.scroller)
      visit(node.scroll_axis)
    end

    def visit_declaration_animation_scroller(node)
    end

    def visit_declaration_animation_timeline_name(node)
    end

    def visit_declaration_animation_timing_function_cubic_bezier(node)
      visit(node.x1)
      visit(node.y1)
      visit(node.x2)
      visit(node.y2)
    end

    def visit_declaration_animation_timing_function_keyword(node)
    end

    def visit_declaration_animation_timing_function_piecewise_linear_function(node)
      visit_list(node.values)
    end

    def visit_declaration_animation_timing_function_piecewise_linear_function_entry(node)
    end

    def visit_declaration_animation_timing_function_steps(node)
    end

    def visit_declaration_animation_view_function(node)
      visit(node.scroll_axis)
      visit(node.inset)
    end

    def visit_declaration_animation_composition(node)
      visit_list(node.values)
    end

    def visit_declaration_animation_delay(node)
      visit_list(node.values)
    end

    def visit_declaration_animation_direction(node)
      visit_list(node.values)
    end

    def visit_declaration_animation_duration(node)
      visit_list(node.values)
    end

    def visit_declaration_animation_duration_value(node)
      visit(node.time)
    end

    def visit_declaration_animation_fill_mode(node)
      visit_list(node.values)
    end

    def visit_declaration_animation_iteration_count(node)
      visit_list(node.values)
    end

    def visit_declaration_animation_name(node)
      visit_list(node.values)
    end

    def visit_declaration_animation_play_state(node)
      visit_list(node.values)
    end

    def visit_declaration_animation_range_end(node)
      visit_list(node.values)
    end

    def visit_declaration_animation_range_start(node)
      visit_list(node.values)
    end

    def visit_declaration_animation_timeline(node)
      visit_list(node.values)
    end

    def visit_declaration_animation_timing_function(node)
      visit_list(node.values)
    end

    def visit_declaration_aspect_ratio(node)
      visit(node.numerator)
      visit(node.denominator)
    end

    def visit_declaration_backdrop_filter(node)
      visit_list(node.values)
    end

    def visit_declaration_backface_visibility(node)
    end

    def visit_declaration_background_attachment(node)
      visit_list(node.values)
    end

    def visit_declaration_background_blend_mode(node)
      visit_list(node.values)
    end

    def visit_declaration_background_clip(node)
      visit_list(node.values)
    end

    def visit_declaration_background_color(node)
      visit(node.color)
    end

    def visit_declaration_background_image(node)
      visit_list(node.values)
    end

    def visit_declaration_background_origin(node)
      visit_list(node.values)
    end

    def visit_declaration_background_position_x(node)
      visit_list(node.values)
    end

    def visit_declaration_background_position_y(node)
      visit_list(node.values)
    end

    def visit_declaration_background_repeat(node)
      visit_list(node.values)
    end

    def visit_declaration_background_repeat_value(node)
    end

    def visit_declaration_background_size(node)
      visit_list(node.values)
    end

    def visit_declaration_background_size_explicit_size(node)
      visit(node.width)
      visit(node.height)
    end

    def visit_declaration_background_size_cover(node)
    end

    def visit_declaration_background_size_contain(node)
    end

    def visit_declaration_background_size_auto(node)
    end

    def visit_declaration_baseline_shift_keyword(node)
    end

    def visit_declaration_baseline_shift_length(node)
      visit(node.value)
    end

    def visit_declaration_baseline_source(node)
    end

    def visit_declaration_block_size(node)
      visit(node.size)
    end

    def visit_declaration_border_block_end_color(node)
      visit(node.color)
    end

    def visit_declaration_border_block_end_style(node)
    end

    def visit_declaration_border_block_end_width(node)
      visit(node.value)
    end

    def visit_declaration_border_block_start_color(node)
      visit(node.color)
    end

    def visit_declaration_border_block_start_style(node)
    end

    def visit_declaration_border_block_start_width(node)
      visit(node.value)
    end

    def visit_declaration_border_bottom_color(node)
      visit(node.color)
    end

    def visit_declaration_border_bottom_left_radius(node)
      visit(node.width)
      visit(node.height)
    end

    def visit_declaration_border_bottom_right_radius(node)
      visit(node.width)
      visit(node.height)
    end

    def visit_declaration_border_bottom_style(node)
    end

    def visit_declaration_border_bottom_width(node)
      visit(node.value)
    end

    def visit_declaration_border_collapse(node)
    end

    def visit_declaration_border_end_end_radius(node)
      visit(node.width)
      visit(node.height)
    end

    def visit_declaration_border_end_start_radius(node)
      visit(node.width)
      visit(node.height)
    end

    def visit_declaration_border_image_outset(node)
      visit(node.top)
      visit(node.right)
      visit(node.bottom)
      visit(node.left)
    end

    def visit_declaration_border_image_repeat(node)
    end

    def visit_declaration_border_image_slice(node)
      visit(node.top)
      visit(node.right)
      visit(node.bottom)
      visit(node.left)
    end

    def visit_declaration_border_image_source(node)
      visit(node.image)
    end

    def visit_declaration_border_image_width(node)
      visit(node.top)
      visit(node.right)
      visit(node.bottom)
      visit(node.left)
    end

    def visit_declaration_border_image_width_auto(node)
    end

    def visit_declaration_border_inline_end_color(node)
      visit(node.color)
    end

    def visit_declaration_border_inline_end_style(node)
    end

    def visit_declaration_border_inline_end_width(node)
      visit(node.value)
    end

    def visit_declaration_border_inline_start_color(node)
      visit(node.color)
    end

    def visit_declaration_border_inline_start_style(node)
    end

    def visit_declaration_border_inline_start_width(node)
      visit(node.value)
    end

    def visit_declaration_border_left_color(node)
      visit(node.color)
    end

    def visit_declaration_border_left_style(node)
    end

    def visit_declaration_border_left_width(node)
      visit(node.value)
    end

    def visit_declaration_border_right_color(node)
      visit(node.color)
    end

    def visit_declaration_border_right_style(node)
    end

    def visit_declaration_border_right_width(node)
      visit(node.value)
    end

    def visit_declaration_border_spacing(node)
      visit(node.horizontal)
      visit(node.vertical)
    end

    def visit_declaration_border_start_end_radius(node)
      visit(node.width)
      visit(node.height)
    end

    def visit_declaration_border_start_start_radius(node)
      visit(node.width)
      visit(node.height)
    end

    def visit_declaration_border_top_color(node)
      visit(node.color)
    end

    def visit_declaration_border_top_left_radius(node)
      visit(node.width)
      visit(node.height)
    end

    def visit_declaration_border_top_right_radius(node)
      visit(node.width)
      visit(node.height)
    end

    def visit_declaration_border_top_style(node)
    end

    def visit_declaration_border_top_width(node)
      visit(node.value)
    end

    def visit_declaration_bottom(node)
      visit(node.value)
    end

    def visit_declaration_box_shadow(node)
      visit_list(node.values)
    end

    def visit_declaration_box_shadow_shadow(node)
      visit(node.color)
      visit(node.horizontal)
      visit(node.vertical)
      visit(node.blur)
      visit(node.spread)
    end

    def visit_declaration_box_sizing(node)
    end

    def visit_declaration_calc(node)
      visit(node.root)
    end

    def visit_declaration_calc_abs(node)
      visit_list(node.children)
    end

    def visit_declaration_calc_anchor_function(node)
      visit(node.target_element)
      visit(node.side)
      visit(node.fallback)
      visit_list(node.children)
    end

    def visit_declaration_calc_anchor_side_keyword(node)
    end

    def visit_declaration_calc_anchor_size_function(node)
      visit(node.target_element)
      visit(node.size)
      visit(node.fallback)
      visit_list(node.children)
    end

    def visit_declaration_calc_anchor_size_keyword(node)
    end

    def visit_declaration_calc_clamp(node)
      visit_list(node.children)
      visit(node.min)
      visit(node.center)
      visit(node.max)
    end

    def visit_declaration_calc_hypot(node)
      visit_list(node.children)
    end

    def visit_declaration_calc_invert(node)
      visit_list(node.children)
    end

    def visit_declaration_calc_min_max(node)
      visit_list(node.children)
    end

    def visit_declaration_calc_mod_rem(node)
      visit_list(node.children)
      visit(node.dividend)
      visit(node.divisor)
    end

    def visit_declaration_calc_negate(node)
      visit_list(node.children)
    end

    def visit_declaration_calc_product(node)
      visit_list(node.children)
    end

    def visit_declaration_calc_round(node)
      visit_list(node.children)
      visit(node.value)
      visit(node.step)
    end

    def visit_declaration_calc_sign(node)
      visit_list(node.children)
    end

    def visit_declaration_calc_sum(node)
      visit_list(node.children)
    end

    def visit_declaration_caption_side(node)
    end

    def visit_declaration_caret_color(node)
      visit(node.color)
    end

    def visit_declaration_channel_keyword(node)
    end

    def visit_declaration_clear(node)
    end

    def visit_declaration_clip(node)
      visit(node.value)
    end

    def visit_declaration_clip_auto(node)
    end

    def visit_declaration_clip_rect(node)
      visit(node.top)
      visit(node.right)
      visit(node.bottom)
      visit(node.left)
    end

    def visit_declaration_clip_length(node)
      visit(node.value)
    end

    def visit_declaration_clip_length_auto(node)
    end

    def visit_declaration_clip_path(node)
      visit(node.value)
    end

    def visit_declaration_clip_path_auto(node)
    end

    def visit_declaration_clip_path_circle(node)
      visit(node.position)
      visit(node.radius)
    end

    def visit_declaration_clip_path_ellipse(node)
      visit(node.position)
      visit(node.x_radius)
      visit(node.y_radius)
    end

    def visit_declaration_clip_path_position(node)
      visit(node.x)
      visit(node.y)
    end

    def visit_declaration_clip_path_position_auto(node)
    end

    def visit_declaration_clip_path_shape_radius_length(node)
      visit(node.value)
    end

    def visit_declaration_clip_path_shape_radius_closest_side(node)
    end

    def visit_declaration_clip_path_shape_radius_farthest_side(node)
    end

    def visit_declaration_clip_path_none(node)
    end

    def visit_declaration_clip_path_url(node)
      visit(node.value)
    end

    def visit_declaration_clip_path_shape(node)
      visit(node.shape)
      visit(node.reference_box)
    end

    def visit_declaration_clip_path_box(node)
      visit(node.reference_box)
    end

    def visit_declaration_clip_path_polygon(node)
      visit_list(node.coordinates)
    end

    def visit_declaration_clip_path_polygon_coord(node)
      visit(node.x)
      visit(node.y)
    end

    def visit_declaration_clip_path_path_or_shape(node)
      visit(node.value)
    end

    def visit_declaration_clip_path_path_function(node)
      visit_list(node.commands)
    end

    def visit_declaration_clip_path_path_command(node)
      visit(node.value)
    end

    def visit_declaration_clip_path_path_command_move(node)
      visit(node.point)
    end

    def visit_declaration_clip_path_path_command_line(node)
      visit(node.point)
    end

    def visit_declaration_clip_path_path_command_h_line(node)
      visit(node.x)
    end

    def visit_declaration_clip_path_path_command_v_line(node)
      visit(node.y)
    end

    def visit_declaration_clip_path_path_command_cubic_curve(node)
      visit(node.control1)
      visit(node.control2)
      visit(node.point)
    end

    def visit_declaration_clip_path_path_command_quad_curve(node)
      visit(node.control1)
      visit(node.point)
    end

    def visit_declaration_clip_path_path_command_smooth_cubic(node)
      visit(node.control2)
      visit(node.point)
    end

    def visit_declaration_clip_path_path_command_smooth_quad(node)
      visit(node.point)
    end

    def visit_declaration_clip_path_path_command_arc(node)
      visit(node.radii)
      visit(node.rotate)
      visit(node.point)
    end

    def visit_declaration_clip_path_path_command_close(node)
    end

    def visit_declaration_clip_path_path_command_end_point_to_position(node)
    end

    def visit_declaration_clip_path_path_command_end_point_by_coordinate(node)
      visit(node.coord)
    end

    def visit_declaration_clip_path_path_coordinate_pair(node)
    end

    def visit_declaration_clip_path_path_axis_end_point_to_position(node)
      visit(node.value)
    end

    def visit_declaration_clip_path_path_axis_end_point_by_coordinate(node)
    end

    def visit_declaration_clip_path_path_control_point_absolute(node)
    end

    def visit_declaration_clip_path_path_control_point_relative(node)
      visit(node.value)
    end

    def visit_declaration_clip_path_path_relative_control_point(node)
      visit(node.coord)
    end

    def visit_declaration_clip_path_path_arc_radii(node)
      visit(node.rx)
      visit(node.ry)
    end

    def visit_declaration_clip_path_rect(node)
      visit(node.value)
    end

    def visit_declaration_clip_path_inset_rect(node)
      visit(node.top)
      visit(node.right)
      visit(node.bottom)
      visit(node.left)
      visit(node.round)
    end

    def visit_declaration_clip_path_xywh_rect(node)
      visit(node.x)
      visit(node.y)
      visit(node.width)
      visit(node.height)
      visit(node.round)
    end

    def visit_declaration_clip_path_rect_function(node)
      visit(node.top)
      visit(node.right)
      visit(node.bottom)
      visit(node.left)
      visit(node.round)
    end

    def visit_declaration_clip_path_border_radius(node)
      visit(node.top_left)
      visit(node.top_right)
      visit(node.bottom_right)
      visit(node.bottom_left)
    end

    def visit_declaration_clip_path_border_corner_radius(node)
      visit(node.width)
      visit(node.height)
    end

    def visit_declaration_clip_path_shape_function(node)
      visit_list(node.commands)
    end

    def visit_declaration_clip_path_shape_command(node)
      visit(node.value)
    end

    def visit_declaration_clip_path_shape_command_move(node)
      visit(node.point)
    end

    def visit_declaration_clip_path_shape_command_line(node)
      visit(node.point)
    end

    def visit_declaration_clip_path_shape_command_h_line(node)
      visit(node.x)
    end

    def visit_declaration_clip_path_shape_command_v_line(node)
      visit(node.y)
    end

    def visit_declaration_clip_path_shape_command_cubic_curve(node)
      visit(node.control1)
      visit(node.control2)
      visit(node.point)
    end

    def visit_declaration_clip_path_shape_command_quad_curve(node)
      visit(node.control1)
      visit(node.point)
    end

    def visit_declaration_clip_path_shape_command_smooth_cubic(node)
      visit(node.control2)
      visit(node.point)
    end

    def visit_declaration_clip_path_shape_command_smooth_quad(node)
      visit(node.point)
    end

    def visit_declaration_clip_path_shape_command_arc(node)
      visit(node.radii)
      visit(node.rotate)
      visit(node.point)
    end

    def visit_declaration_clip_path_shape_command_close(node)
    end

    def visit_declaration_clip_path_command_end_point_to_position(node)
      visit(node.horizontal)
      visit(node.vertical)
    end

    def visit_declaration_clip_path_command_end_point_by_coordinate(node)
      visit(node.coord)
    end

    def visit_declaration_clip_path_coordinate_pair(node)
      visit(node.x)
      visit(node.y)
    end

    def visit_declaration_clip_path_axis_end_point_to_position(node)
      visit(node.value)
    end

    def visit_declaration_clip_path_axis_end_point_by_coordinate(node)
      visit(node.value)
    end

    def visit_declaration_clip_path_control_point_absolute(node)
      visit(node.value)
    end

    def visit_declaration_clip_path_control_point_relative(node)
      visit(node.value)
    end

    def visit_declaration_clip_path_relative_control_point(node)
      visit(node.coord)
    end

    def visit_declaration_clip_path_arc_radii(node)
      visit(node.rx)
      visit(node.ry)
    end

    def visit_declaration_clip_path_element_dependent(node)
    end

    def visit_declaration_clip_path_fill_box(node)
    end

    def visit_declaration_clip_path_stroke_box(node)
    end

    def visit_declaration_clip_path_view_box(node)
    end

    def visit_declaration_clip_path_margin_box(node)
    end

    def visit_declaration_clip_path_border_box(node)
    end

    def visit_declaration_clip_path_padding_box(node)
    end

    def visit_declaration_clip_path_content_box(node)
    end

    def visit_declaration_color(node)
      visit(node.color)
    end

    def visit_declaration_color_absolute(node)
      visit(node.color)
      visit(node.authored)
    end

    def visit_declaration_color_absolute_color(node)
      visit(node.components)
    end

    def visit_declaration_color_auto(node)
    end

    def visit_declaration_color_color_components(node)
    end

    def visit_declaration_color_color_function(node)
      visit(node.origin_color)
      visit_list(node.components)
      visit(node.alpha)
      visit(node.color_space)
    end

    def visit_declaration_color_color_function_component(node)
      visit(node.value)
      visit(node.channel_keyword)
      visit(node.calc)
    end

    def visit_declaration_color_color_interpolation_method(node)
    end

    def visit_declaration_color_color_mix(node)
      visit(node.interpolation)
      visit_list(node.items)
    end

    def visit_declaration_color_color_mix_item(node)
      visit(node.color)
      visit(node.percentage)
    end

    def visit_declaration_color_current_color(node)
    end

    def visit_declaration_color_light_dark(node)
      visit(node.light)
      visit(node.dark)
    end

    def visit_declaration_color_system_color(node)
    end

    def visit_declaration_color_scheme(node)
      visit_list(node.values)
    end

    def visit_declaration_column_count_auto(node)
    end

    def visit_declaration_column_count_integer(node)
    end

    def visit_declaration_column_gap(node)
      visit(node.value)
    end

    def visit_declaration_column_gap_normal(node)
    end

    def visit_declaration_column_gap_length_percentage(node)
      visit(node.value)
    end

    def visit_declaration_column_span(node)
    end

    def visit_declaration_column_width(node)
      visit(node.value)
    end

    def visit_declaration_column_width_auto(node)
    end

    def visit_declaration_column_width_length(node)
      visit(node.value)
    end

    def visit_declaration_contain(node)
      visit_list(node.values)
    end

    def visit_declaration_container_name(node)
      visit_list(node.values)
    end

    def visit_declaration_container_type(node)
      visit_list(node.values)
    end

    def visit_declaration_content(node)
      visit(node.value)
    end

    def visit_declaration_content_normal(node)
    end

    def visit_declaration_content_none(node)
    end

    def visit_declaration_content_items(node)
      visit_list(node.items)
    end

    def visit_declaration_content_item_string(node)
    end

    def visit_declaration_content_item_counter(node)
    end

    def visit_declaration_content_item_counters(node)
    end

    def visit_declaration_content_item_attr(node)
    end

    def visit_declaration_content_item_image(node)
      visit(node.image)
    end

    def visit_declaration_content_counter_style_symbols(node)
    end

    def visit_declaration_counter_increment(node)
      visit_list(node.values)
    end

    def visit_declaration_counter_increment_counter(node)
    end

    def visit_declaration_counter_reset(node)
      visit_list(node.values)
    end

    def visit_declaration_counter_reset_counter(node)
    end

    def visit_declaration_css_wide_keyword(node)
    end

    def visit_declaration_cursor(node)
      visit_list(node.images)
    end

    def visit_declaration_cursor_image(node)
      visit(node.image)
      visit(node.hotspot_x)
      visit(node.hotspot_y)
    end

    def visit_declaration_custom(node)
      visit(node.name)
      visit(node.value)
    end

    def visit_declaration_custom_unparsed(node)
    end

    def visit_declaration_custom_parsed(node)
    end

    def visit_declaration_custom_css_wide_keyword(node)
    end

    def visit_declaration_direction(node)
    end

    def visit_declaration_display(node)
    end

    def visit_declaration_empty_cells(node)
    end

    def visit_declaration_filter(node)
      visit_list(node.values)
    end

    def visit_declaration_filter_blur(node)
      visit(node.value)
    end

    def visit_declaration_filter_brightness(node)
      visit(node.value)
    end

    def visit_declaration_filter_contrast(node)
      visit(node.value)
    end

    def visit_declaration_filter_drop_shadow(node)
      visit(node.color)
      visit(node.horizontal)
      visit(node.vertical)
      visit(node.blur)
    end

    def visit_declaration_filter_grayscale(node)
      visit(node.value)
    end

    def visit_declaration_filter_hue_rotate(node)
      visit(node.value)
    end

    def visit_declaration_filter_invert(node)
      visit(node.value)
    end

    def visit_declaration_filter_opacity(node)
      visit(node.value)
    end

    def visit_declaration_filter_saturate(node)
      visit(node.value)
    end

    def visit_declaration_filter_sepia(node)
      visit(node.value)
    end

    def visit_declaration_flex_basis(node)
      visit(node.value)
    end

    def visit_declaration_flex_basis_content(node)
    end

    def visit_declaration_flex_basis_size(node)
      visit(node.value)
    end

    def visit_declaration_flex_direction(node)
    end

    def visit_declaration_flex_grow(node)
      visit(node.value)
    end

    def visit_declaration_flex_shrink(node)
      visit(node.value)
    end

    def visit_declaration_flex_wrap(node)
    end

    def visit_declaration_float(node)
    end

    def visit_declaration_font_family(node)
      visit(node.value)
    end

    def visit_declaration_font_family_values(node)
      visit_list(node.values)
    end

    def visit_declaration_font_family_system(node)
    end

    def visit_declaration_font_family_name(node)
    end

    def visit_declaration_font_family_generic(node)
    end

    def visit_declaration_font_language_override(node)
      visit(node.value)
    end

    def visit_declaration_font_optical_sizing(node)
    end

    def visit_declaration_font_size(node)
      visit(node.value)
    end

    def visit_declaration_font_size_length(node)
      visit(node.value)
    end

    def visit_declaration_font_size_keyword(node)
    end

    def visit_declaration_font_size_smaller(node)
    end

    def visit_declaration_font_size_larger(node)
    end

    def visit_declaration_font_size_system(node)
    end

    def visit_declaration_font_stretch(node)
      visit(node.value)
    end

    def visit_declaration_font_stretch_stretch(node)
      visit(node.value)
    end

    def visit_declaration_font_stretch_keyword(node)
    end

    def visit_declaration_font_stretch_system(node)
    end

    def visit_declaration_font_style(node)
      visit(node.value)
    end

    def visit_declaration_font_style_normal(node)
    end

    def visit_declaration_font_style_italic(node)
    end

    def visit_declaration_font_style_oblique(node)
      visit(node.angle)
    end

    def visit_declaration_font_style_system(node)
    end

    def visit_declaration_font_synthesis_weight(node)
    end

    def visit_declaration_font_variant_caps(node)
    end

    def visit_declaration_font_variation_settings(node)
      visit_list(node.values)
    end

    def visit_declaration_font_variation_settings_setting(node)
      visit(node.value)
    end

    def visit_declaration_font_weight(node)
      visit(node.value)
    end

    def visit_declaration_font_weight_absolute(node)
      visit(node.value)
    end

    def visit_declaration_font_weight_bolder(node)
    end

    def visit_declaration_font_weight_lighter(node)
    end

    def visit_declaration_font_weight_system(node)
    end

    def visit_declaration_grid_auto_columns(node)
      visit_list(node.values)
    end

    def visit_declaration_grid_auto_flow(node)
    end

    def visit_declaration_grid_auto_rows(node)
      visit_list(node.values)
    end

    def visit_declaration_grid_column_end(node)
    end

    def visit_declaration_grid_column_start(node)
    end

    def visit_declaration_grid_row_end(node)
    end

    def visit_declaration_grid_row_start(node)
    end

    def visit_declaration_grid_template_none(node)
    end

    def visit_declaration_grid_template_masonry(node)
    end

    def visit_declaration_grid_template_track_list(node)
      visit_list(node.values)
      visit_list(node.line_names)
    end

    def visit_declaration_grid_template_track_list_value(node)
      visit(node.value)
    end

    def visit_declaration_grid_template_track_list_value_track_repeat(node)
      visit(node.count)
      visit_list(node.line_names)
      visit_list(node.track_sizes)
    end

    def visit_declaration_grid_template_repeat_count_number(node)
    end

    def visit_declaration_grid_template_repeat_count_auto_fill(node)
    end

    def visit_declaration_grid_template_repeat_count_auto_fit(node)
    end

    def visit_declaration_grid_template_subgrid(node)
      visit_list(node.line_names)
    end

    def visit_declaration_grid_template_line_name_list_value(node)
      visit(node.value)
    end

    def visit_declaration_grid_template_line_names(node)
      visit_list(node.names)
    end

    def visit_declaration_grid_template_line_name_repeat(node)
      visit(node.count)
      visit_list(node.line_names)
    end

    def visit_declaration_grid_template_areas(node)
      visit(node.areas)
    end

    def visit_declaration_grid_template_areas_none(node)
    end

    def visit_declaration_grid_template_areas_area_list(node)
      visit_list(node.strings)
      visit_list(node.areas)
    end

    def visit_declaration_grid_template_areas_area(node)
    end

    def visit_declaration_grid_template_columns(node)
      visit(node.value)
    end

    def visit_declaration_grid_template_rows(node)
      visit(node.value)
    end

    def visit_declaration_height(node)
      visit(node.size)
    end

    def visit_declaration_image_center_horizontal_position_component(node)
    end

    def visit_declaration_image_length_horizontal_position_component(node)
      visit(node.length)
    end

    def visit_declaration_image_side_horizontal_position_component(node)
      visit(node.offset)
    end

    def visit_declaration_image_angle_or_percentage(node)
      visit(node.value)
    end

    def visit_declaration_image_radius_circle(node)
      visit(node.length)
    end

    def visit_declaration_image_extent_circle(node)
    end

    def visit_declaration_image_conic_gradient(node)
      visit(node.angle)
      visit(node.position)
      visit(node.color_interpolation_method)
      visit_list(node.items)
    end

    def visit_declaration_image_cross_fade(node)
      visit_list(node.elements)
    end

    def visit_declaration_image_cross_fade_element(node)
      visit(node.percent)
      visit(node.image)
    end

    def visit_declaration_image_cross_fade_image(node)
      visit(node.image)
    end

    def visit_declaration_image_cross_fade_color(node)
      visit(node.color)
    end

    def visit_declaration_image_radii_ellipse(node)
      visit(node.x)
      visit(node.y)
    end

    def visit_declaration_image_extent_ellipse(node)
    end

    def visit_declaration_image_ending_shape(node)
      visit(node.value)
    end

    def visit_declaration_image_gradient_simple_color_stop_angle(node)
      visit(node.color)
    end

    def visit_declaration_image_gradient_complex_color_stop_angle(node)
      visit(node.color)
      visit(node.position)
    end

    def visit_declaration_image_gradient_interpolation_hint_angle(node)
      visit(node.position)
    end

    def visit_declaration_image_gradient_simple_color_stop_length(node)
      visit(node.color)
    end

    def visit_declaration_image_gradient_complex_color_stop_length(node)
      visit(node.color)
      visit(node.position)
    end

    def visit_declaration_image_gradient_interpolation_hint_length(node)
      visit(node.position)
    end

    def visit_declaration_image_image_set(node)
      visit_list(node.items)
    end

    def visit_declaration_image_image_set_item(node)
      visit(node.image)
      visit(node.resolution)
      visit(node.mime_type)
    end

    def visit_declaration_image_light_dark(node)
      visit(node.light)
      visit(node.dark)
    end

    def visit_declaration_image_angle_line_direction(node)
      visit(node.angle)
    end

    def visit_declaration_image_horizontal_line_direction(node)
    end

    def visit_declaration_image_vertical_line_direction(node)
    end

    def visit_declaration_image_corner_line_direction(node)
    end

    def visit_declaration_image_linear_gradient(node)
      visit(node.direction)
      visit(node.color_interpolation_method)
      visit_list(node.items)
    end

    def visit_declaration_image_none(node)
    end

    def visit_declaration_image_position(node)
      visit(node.horizontal_position)
      visit(node.vertical_position)
    end

    def visit_declaration_image_radial_gradient(node)
      visit(node.shape)
      visit(node.position)
      visit(node.color_interpolation_method)
      visit_list(node.items)
    end

    def visit_declaration_image_url(node)
      visit(node.resolved)
      visit(node.original)
    end

    def visit_declaration_image_center_vertical_position_component(node)
    end

    def visit_declaration_image_length_vertical_position_component(node)
      visit(node.length)
    end

    def visit_declaration_image_side_vertical_position_component(node)
      visit(node.offset)
    end

    def visit_declaration_image_rendering(node)
    end

    def visit_declaration_inline_size(node)
      visit(node.size)
    end

    def visit_declaration_inset_auto(node)
    end

    def visit_declaration_inset_length_percentage(node)
      visit(node.value)
    end

    def visit_declaration_inset_anchor_containing_calc_function(node)
      visit(node.value)
    end

    def visit_declaration_inset_anchor_function(node)
      visit(node.value)
    end

    def visit_declaration_inset_anchor_size_function(node)
      visit(node.value)
    end

    def visit_declaration_inset_block_end(node)
      visit(node.value)
    end

    def visit_declaration_inset_block_start(node)
      visit(node.value)
    end

    def visit_declaration_inset_inline_end(node)
      visit(node.value)
    end

    def visit_declaration_inset_inline_start(node)
      visit(node.value)
    end

    def visit_declaration_isolation(node)
    end

    def visit_declaration_justify_content(node)
    end

    def visit_declaration_justify_items(node)
    end

    def visit_declaration_justify_self(node)
    end

    def visit_declaration_left(node)
      visit(node.value)
    end

    def visit_declaration_length_absolute(node)
    end

    def visit_declaration_length_font_relative(node)
    end

    def visit_declaration_length_viewport_percentage(node)
    end

    def visit_declaration_length_container_relative(node)
    end

    def visit_declaration_length_character_width(node)
    end

    def visit_declaration_letter_spacing(node)
      visit(node.value)
    end

    def visit_declaration_letter_spacing_normal(node)
    end

    def visit_declaration_line_break(node)
    end

    def visit_declaration_line_height(node)
      visit(node.value)
    end

    def visit_declaration_line_height_normal(node)
    end

    def visit_declaration_list_style_image(node)
      visit(node.image)
    end

    def visit_declaration_list_style_position(node)
    end

    def visit_declaration_list_style_type(node)
      visit(node.value)
    end

    def visit_declaration_list_style_type_none(node)
    end

    def visit_declaration_list_style_type_name(node)
    end

    def visit_declaration_list_style_type_string(node)
    end

    def visit_declaration_list_style_type_symbols(node)
    end

    def visit_declaration_margin_auto(node)
    end

    def visit_declaration_margin_anchor_containing_calc_function(node)
      visit(node.value)
    end

    def visit_declaration_margin_anchor_size_function(node)
      visit(node.target_element)
      visit(node.size)
      visit(node.fallback)
    end

    def visit_declaration_margin_block_end(node)
      visit(node.value)
    end

    def visit_declaration_margin_block_start(node)
      visit(node.value)
    end

    def visit_declaration_margin_bottom(node)
      visit(node.value)
    end

    def visit_declaration_margin_inline_end(node)
      visit(node.value)
    end

    def visit_declaration_margin_inline_start(node)
      visit(node.value)
    end

    def visit_declaration_margin_left(node)
      visit(node.value)
    end

    def visit_declaration_margin_right(node)
      visit(node.value)
    end

    def visit_declaration_margin_top(node)
      visit(node.value)
    end

    def visit_declaration_mask_image(node)
      visit_list(node.values)
    end

    def visit_declaration_max_block_size(node)
      visit(node.size)
    end

    def visit_declaration_max_height(node)
      visit(node.size)
    end

    def visit_declaration_max_inline_size(node)
      visit(node.size)
    end

    def visit_declaration_max_width(node)
      visit(node.size)
    end

    def visit_declaration_min_block_size(node)
      visit(node.size)
    end

    def visit_declaration_min_height(node)
      visit(node.size)
    end

    def visit_declaration_min_inline_size(node)
      visit(node.size)
    end

    def visit_declaration_min_width(node)
      visit(node.size)
    end

    def visit_declaration_mix_blend_mode(node)
    end

    def visit_declaration_number(node)
    end

    def visit_declaration_object_fit(node)
    end

    def visit_declaration_object_position(node)
      visit(node.horizontal)
      visit(node.vertical)
    end

    def visit_declaration_offset_path(node)
      visit(node.value)
    end

    def visit_declaration_offset_path_none(node)
    end

    def visit_declaration_offset_path_coord_box(node)
    end

    def visit_declaration_offset_path_function(node)
      visit(node.path)
    end

    def visit_declaration_offset_path_ray(node)
      visit(node.angle)
      visit(node.position)
    end

    def visit_declaration_offset_path_url(node)
      visit(node.value)
    end

    def visit_declaration_offset_path_position_auto(node)
    end

    def visit_declaration_offset_path_position(node)
      visit(node.horizontal)
      visit(node.vertical)
    end

    def visit_declaration_opacity(node)
      visit(node.value)
    end

    def visit_declaration_order(node)
    end

    def visit_declaration_outline_color(node)
      visit(node.color)
    end

    def visit_declaration_outline_offset(node)
      visit(node.value)
    end

    def visit_declaration_outline_style(node)
    end

    def visit_declaration_outline_width(node)
      visit(node.value)
    end

    def visit_declaration_overflow_block(node)
    end

    def visit_declaration_overflow_clip_margin(node)
      visit(node.offset)
    end

    def visit_declaration_overflow_inline(node)
    end

    def visit_declaration_overflow_wrap(node)
    end

    def visit_declaration_overflow_x(node)
    end

    def visit_declaration_overflow_y(node)
    end

    def visit_declaration_padding_block_end(node)
      visit(node.value)
    end

    def visit_declaration_padding_block_start(node)
      visit(node.value)
    end

    def visit_declaration_padding_bottom(node)
      visit(node.value)
    end

    def visit_declaration_padding_inline_end(node)
      visit(node.value)
    end

    def visit_declaration_padding_inline_start(node)
      visit(node.value)
    end

    def visit_declaration_padding_left(node)
      visit(node.value)
    end

    def visit_declaration_padding_right(node)
      visit(node.value)
    end

    def visit_declaration_padding_top(node)
      visit(node.value)
    end

    def visit_declaration_percentage(node)
    end

    def visit_declaration_perspective(node)
      visit(node.value)
    end

    def visit_declaration_perspective_none(node)
    end

    def visit_declaration_perspective_length(node)
      visit(node.value)
    end

    def visit_declaration_perspective_origin(node)
      visit(node.horizontal)
      visit(node.vertical)
    end

    def visit_declaration_pointer_events(node)
    end

    def visit_declaration_position(node)
    end

    def visit_declaration_position_area(node)
    end

    def visit_declaration_position_try_fallbacks(node)
      visit_list(node.values)
    end

    def visit_declaration_position_try_fallbacks_ident_and_or_tactic(node)
    end

    def visit_declaration_position_try_fallbacks_position_area(node)
    end

    def visit_declaration_quotes(node)
      visit(node.value)
    end

    def visit_declaration_quotes_auto(node)
    end

    def visit_declaration_quotes_none(node)
    end

    def visit_declaration_quotes_quote_list(node)
      visit_list(node.values)
    end

    def visit_declaration_quotes_quote_pair(node)
    end

    def visit_declaration_resolution(node)
    end

    def visit_declaration_right(node)
      visit(node.value)
    end

    def visit_declaration_rotate(node)
      visit(node.value)
    end

    def visit_declaration_rotate_none(node)
    end

    def visit_declaration_rotate_rotate3_d(node)
      visit(node.x)
      visit(node.y)
      visit(node.z)
      visit(node.angle)
    end

    def visit_declaration_row_gap(node)
      visit(node.value)
    end

    def visit_declaration_row_gap_normal(node)
    end

    def visit_declaration_row_gap_length_percentage(node)
      visit(node.value)
    end

    def visit_declaration_scale(node)
      visit(node.value)
    end

    def visit_declaration_scale_none(node)
    end

    def visit_declaration_scale_coords(node)
      visit(node.x)
      visit(node.y)
      visit(node.z)
    end

    def visit_declaration_servo_overflow_clip_box(node)
    end

    def visit_declaration_servo_top_layer(node)
    end

    def visit_declaration_size_anchor_max_size_function(node)
      visit(node.target_element)
      visit(node.size)
      visit(node.fallback)
    end

    def visit_declaration_size_anchor_size_function(node)
      visit(node.target_element)
      visit(node.size)
      visit(node.fallback)
    end

    def visit_declaration_size_length_percentage(node)
      visit(node.value)
    end

    def visit_declaration_size_none(node)
    end

    def visit_declaration_size_auto(node)
    end

    def visit_declaration_size_max_content(node)
    end

    def visit_declaration_size_min_content(node)
    end

    def visit_declaration_size_fit_content(node)
    end

    def visit_declaration_size_webkit_fill_available(node)
    end

    def visit_declaration_size_stretch(node)
    end

    def visit_declaration_size_fit_content_function(node)
      visit(node.value)
    end

    def visit_declaration_size_anchor_containing_calc_function(node)
      visit(node.value)
    end

    def visit_declaration_table_layout(node)
    end

    def visit_declaration_text_align(node)
    end

    def visit_declaration_text_align_last(node)
    end

    def visit_declaration_text_decoration_color(node)
      visit(node.color)
    end

    def visit_declaration_text_decoration_line(node)
      visit_list(node.values)
    end

    def visit_declaration_text_decoration_style(node)
    end

    def visit_declaration_text_indent(node)
      visit(node.length)
    end

    def visit_declaration_text_justify(node)
    end

    def visit_declaration_text_overflow(node)
      visit(node.first)
      visit(node.second)
    end

    def visit_declaration_text_overflow_clip(node)
    end

    def visit_declaration_text_overflow_ellipsis(node)
    end

    def visit_declaration_text_overflow_string(node)
    end

    def visit_declaration_text_rendering(node)
    end

    def visit_declaration_text_shadow(node)
      visit_list(node.values)
    end

    def visit_declaration_text_shadow_shadow(node)
      visit(node.color)
      visit(node.horizontal)
      visit(node.vertical)
      visit(node.blur)
    end

    def visit_declaration_text_transform(node)
      visit_list(node.values)
    end

    def visit_declaration_text_wrap_mode(node)
    end

    def visit_declaration_time(node)
    end

    def visit_declaration_top(node)
      visit(node.value)
    end

    def visit_declaration_track_breadth_length_percentage(node)
      visit(node.value)
    end

    def visit_declaration_track_breadth_fr(node)
    end

    def visit_declaration_track_breadth_auto(node)
    end

    def visit_declaration_track_breadth_min_content(node)
    end

    def visit_declaration_track_breadth_max_content(node)
    end

    def visit_declaration_track_size_minmax(node)
      visit(node.min)
      visit(node.max)
    end

    def visit_declaration_track_size_fit_content(node)
      visit(node.value)
    end

    def visit_declaration_transform(node)
      visit_list(node.operations)
    end

    def visit_declaration_transform_matrix(node)
      visit(node.a)
      visit(node.b)
      visit(node.c)
      visit(node.d)
      visit(node.e)
      visit(node.f)
    end

    def visit_declaration_transform_matrix3_d(node)
      visit(node.m11)
      visit(node.m12)
      visit(node.m13)
      visit(node.m14)
      visit(node.m21)
      visit(node.m22)
      visit(node.m23)
      visit(node.m24)
      visit(node.m31)
      visit(node.m32)
      visit(node.m33)
      visit(node.m34)
      visit(node.m41)
      visit(node.m42)
      visit(node.m43)
      visit(node.m44)
    end

    def visit_declaration_transform_skew(node)
      visit(node.x)
      visit(node.y)
    end

    def visit_declaration_transform_skew_x(node)
      visit(node.angle)
    end

    def visit_declaration_transform_skew_y(node)
      visit(node.angle)
    end

    def visit_declaration_transform_translate(node)
      visit(node.x)
      visit(node.y)
    end

    def visit_declaration_transform_translate_x(node)
      visit(node.value)
    end

    def visit_declaration_transform_translate_y(node)
      visit(node.value)
    end

    def visit_declaration_transform_translate_z(node)
      visit(node.value)
    end

    def visit_declaration_transform_translate3_d(node)
      visit(node.x)
      visit(node.y)
      visit(node.z)
    end

    def visit_declaration_transform_scale(node)
      visit(node.x)
      visit(node.y)
    end

    def visit_declaration_transform_scale_x(node)
      visit(node.value)
    end

    def visit_declaration_transform_scale_y(node)
      visit(node.value)
    end

    def visit_declaration_transform_scale_z(node)
      visit(node.value)
    end

    def visit_declaration_transform_scale3_d(node)
      visit(node.x)
      visit(node.y)
      visit(node.z)
    end

    def visit_declaration_transform_rotate(node)
      visit(node.angle)
    end

    def visit_declaration_transform_rotate_x(node)
      visit(node.angle)
    end

    def visit_declaration_transform_rotate_y(node)
      visit(node.angle)
    end

    def visit_declaration_transform_rotate_z(node)
      visit(node.angle)
    end

    def visit_declaration_transform_rotate3_d(node)
      visit(node.x)
      visit(node.y)
      visit(node.z)
      visit(node.angle)
    end

    def visit_declaration_transform_perspective(node)
      visit(node.value)
    end

    def visit_declaration_transform_perspective_none(node)
    end

    def visit_declaration_transform_perspective_length(node)
      visit(node.value)
    end

    def visit_declaration_transform_interpolate_matrix(node)
      visit(node.from_list)
      visit(node.to_list)
      visit(node.progress)
    end

    def visit_declaration_transform_accumulate_matrix(node)
      visit(node.from_list)
      visit(node.to_list)
    end

    def visit_declaration_transform_origin(node)
      visit(node.horizontal)
      visit(node.vertical)
      visit(node.depth)
    end

    def visit_declaration_transform_origin_center(node)
    end

    def visit_declaration_transform_origin_side_horizontal_component(node)
    end

    def visit_declaration_transform_origin_side_vertical_component(node)
    end

    def visit_declaration_transform_style(node)
    end

    def visit_declaration_transition_behavior(node)
      visit_list(node.values)
    end

    def visit_declaration_transition_delay(node)
      visit_list(node.values)
    end

    def visit_declaration_transition_duration(node)
      visit_list(node.values)
    end

    def visit_declaration_transition_property(node)
      visit_list(node.values)
    end

    def visit_declaration_transition_property_non_custom(node)
    end

    def visit_declaration_transition_property_custom(node)
    end

    def visit_declaration_transition_property_unsupported(node)
    end

    def visit_declaration_transition_timing_function(node)
      visit_list(node.values)
    end

    def visit_declaration_translate(node)
      visit(node.value)
    end

    def visit_declaration_translate_none(node)
    end

    def visit_declaration_translate_coords(node)
      visit(node.x)
      visit(node.y)
      visit(node.z)
    end

    def visit_declaration_unicode_bidi(node)
    end

    def visit_declaration_view_transition_class(node)
      visit_list(node.values)
    end

    def visit_declaration_view_transition_name(node)
      visit(node.name)
    end

    def visit_declaration_visibility(node)
    end

    def visit_declaration_webkit_text_security(node)
    end

    def visit_declaration_white_space_collapse(node)
    end

    def visit_declaration_width(node)
      visit(node.size)
    end

    def visit_declaration_will_change(node)
      visit_list(node.values)
    end

    def visit_declaration_with_variables(node)
      visit(node.value)
    end

    def visit_declaration_word_break(node)
    end

    def visit_declaration_word_spacing(node)
      visit(node.value)
    end

    def visit_declaration_word_spacing_normal(node)
    end

    def visit_declaration_writing_mode(node)
    end

    def visit_declaration_x_lang(node)
      visit(node.value)
    end

    def visit_declaration_z_index(node)
      visit(node.value)
    end

    def visit_declaration_z_index_auto(node)
    end

    def visit_declaration_z_index_integer(node)
    end

    def visit_declaration_zoom(node)
      visit(node.value)
    end

    def visit_declaration_zoom_normal(node)
    end

    def visit_declaration_zoom_document(node)
    end

    def visit_declaration_zoom_value(node)
      visit(node.value)
    end

    def visit_selector(node)
      visit_list(node.children)
    end

    def visit_selector_an_plus_b(node)
    end

    def visit_selector_attribute_in_no_namespace(node)
    end

    def visit_selector_attribute_in_no_namespace_exists(node)
    end

    def visit_selector_attribute_other(node)
    end

    def visit_selector_klass(node)
    end

    def visit_selector_combinator(node)
    end

    def visit_selector_default_namespace(node)
    end

    def visit_selector_has(node)
    end

    def visit_selector_host(node)
    end

    def visit_selector_id(node)
    end

    def visit_selector_is(node)
    end

    def visit_selector_local_name(node)
    end

    def visit_selector_namespace(node)
    end

    def visit_selector_negation(node)
    end

    def visit_selector_non_ts_pseudo_class(node)
    end

    def visit_selector_nth(node)
    end

    def visit_selector_nth_of(node)
    end

    def visit_selector_part(node)
    end

    def visit_selector_slotted(node)
    end

    def visit_selector_specific_namespace_constraint(node)
    end

    def visit_selector_where(node)
    end

    def visit_relative_selector(node)
      visit(node.selector)
    end

    def visit_unimplemented_rule(node)
    end

    def visit_style_rule(node)
      visit_list(node.selectors)
      visit_list(node.declarations)
    end

    def visit_style_query_not(node)
      visit(node.style_query)
    end

    def visit_style_query_operation(node)
      visit_list(node.style_query)
    end

    def visit_style_query_in_parens(node)
      visit(node.style_query)
    end

    def visit_style_query_style_feature(node)
    end

    def visit_style_query_generally_enclosed(node)
    end

    def visit_media_rule(node)
      visit_list(node.media_queries)
      visit_list(node.rules)
    end

    def visit_media_query(node)
      visit(node.media_type)
      visit(node.query_condition)
    end

    def visit_media_query_query_condition_feature_expression(node)
    end

    def visit_media_query_query_condition_custom(node)
    end

    def visit_media_query_query_condition_operation(node)
      visit_list(node.query_conditions)
    end

    def visit_media_query_query_condition_in_parens(node)
      visit(node.query_condition)
    end

    def visit_media_query_query_condition_style(node)
      visit(node.style_query)
    end

    def visit_media_query_query_condition_generally_enclosed(node)
    end

    def visit_media_type_all(node)
    end

    def visit_media_type_concrete(node)
    end

    def visit_font_face_rule(node)
      visit(node.ascent_override)
      visit(node.descent_override)
      visit(node.display)
      visit(node.family)
      visit(node.font_face)
      visit(node.line_gap_override)
      visit(node.size_adjust)
      visit(node.font_stretch_range)
      visit(node.style)
      visit(node.weight)
    end

    def visit_font_face(node)
      visit(node.family)
      visit_list(node.sources)
    end

    def visit_font_metrics_override(node)
    end

    def visit_font_metrics_override_normal(node)
    end

    def visit_font_family_name(node)
    end

    def visit_font_source_format_keyword(node)
    end

    def visit_font_source_format_string(node)
    end

    def visit_font_source_url(node)
      visit(node.format_hint)
    end

    def visit_font_source_local(node)
      visit(node.family_name)
    end

    def visit_unicode_range(node)
    end

    def visit_system_font(node)
    end

    def visit_font_stretch_range(node)
      visit(node.begin)
      visit(node.end)
    end

    def visit_font_stretch_stretch(node)
    end

    def visit_font_stretch_keyword(node)
    end

    def visit_font_style_italic(node)
    end

    def visit_font_style_oblique(node)
      visit(node.angle1)
      visit(node.angle2)
    end

    def visit_font_weight_normal(node)
    end

    def visit_font_weight_range(node)
      visit(node.begin)
      visit(node.end)
    end
  end
end
