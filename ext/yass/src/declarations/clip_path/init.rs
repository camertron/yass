use magnus::{Error, Module, RModule, Ruby, method};

use super::{
    YClipPath,
    YClipPathBox,
    YClipPathShape,
    YClipPathUrl,
    circle_ellipse::*,
    path::*,
    rect::*,
    shape::*,
};

pub fn init(ruby: &Ruby, declarations_module: &RModule) -> Result<(), Error> {
    let clip_path_class = declarations_module.define_class("ClipPath", ruby.class_object())?;
    clip_path_class.define_method("value", method!(YClipPath::value, 0))?;

    let clip_path_shape_class = clip_path_class.define_class("Shape", ruby.class_object())?;
    clip_path_shape_class.define_method("shape", method!(YClipPathShape::shape, 0))?;
    clip_path_shape_class.define_method("reference_box", method!(YClipPathShape::reference_box, 0))?;

    let clip_path_box_class = clip_path_class.define_class("Box", ruby.class_object())?;
    clip_path_box_class.define_method("reference_box", method!(YClipPathBox::reference_box, 0))?;

    let clip_path_url_class = clip_path_class.define_class("Url", ruby.class_object())?;
    clip_path_url_class.define_method("value", method!(YClipPathUrl::value, 0))?;

    let clip_path_rect_class = clip_path_class.define_class("Rect", ruby.class_object())?;
    clip_path_rect_class.define_method("value", method!(YClipPathRect::value, 0))?;

    let clip_path_inset_rect_class = clip_path_class.define_class("InsetRect", ruby.class_object())?;
    clip_path_inset_rect_class.define_method("top", method!(YClipPathInsetRect::top, 0))?;
    clip_path_inset_rect_class.define_method("right", method!(YClipPathInsetRect::right, 0))?;
    clip_path_inset_rect_class.define_method("bottom", method!(YClipPathInsetRect::bottom, 0))?;
    clip_path_inset_rect_class.define_method("left", method!(YClipPathInsetRect::left, 0))?;
    clip_path_inset_rect_class.define_method("round", method!(YClipPathInsetRect::round, 0))?;

    let clip_path_border_radius_class = clip_path_class.define_class("BorderRadius", ruby.class_object())?;
    clip_path_border_radius_class.define_method("top_left", method!(YClipPathBorderRadius::top_left, 0))?;
    clip_path_border_radius_class.define_method("top_right", method!(YClipPathBorderRadius::top_right, 0))?;
    clip_path_border_radius_class.define_method("bottom_right", method!(YClipPathBorderRadius::bottom_right, 0))?;
    clip_path_border_radius_class.define_method("bottom_left", method!(YClipPathBorderRadius::bottom_left, 0))?;

    let clip_path_border_corner_radius_class = clip_path_class.define_class("BorderCornerRadius", ruby.class_object())?;
    clip_path_border_corner_radius_class.define_method("width", method!(YClipPathBorderCornerRadius::width, 0))?;
    clip_path_border_corner_radius_class.define_method("height", method!(YClipPathBorderCornerRadius::height, 0))?;

    let clip_path_xywh_rect_class = clip_path_class.define_class("XywhRect", ruby.class_object())?;
    clip_path_xywh_rect_class.define_method("x", method!(YClipPathXywhRect::x, 0))?;
    clip_path_xywh_rect_class.define_method("y", method!(YClipPathXywhRect::y, 0))?;
    clip_path_xywh_rect_class.define_method("width", method!(YClipPathXywhRect::width, 0))?;
    clip_path_xywh_rect_class.define_method("height", method!(YClipPathXywhRect::height, 0))?;
    clip_path_xywh_rect_class.define_method("round", method!(YClipPathXywhRect::round, 0))?;

    let clip_path_rect_function_class = clip_path_class.define_class("RectFunction", ruby.class_object())?;
    clip_path_rect_function_class.define_method("top", method!(YClipPathRectFunction::top, 0))?;
    clip_path_rect_function_class.define_method("right", method!(YClipPathRectFunction::right, 0))?;
    clip_path_rect_function_class.define_method("bottom", method!(YClipPathRectFunction::bottom, 0))?;
    clip_path_rect_function_class.define_method("left", method!(YClipPathRectFunction::left, 0))?;
    clip_path_rect_function_class.define_method("round", method!(YClipPathRectFunction::round, 0))?;

    let clip_path_circle_class = clip_path_class.define_class("Circle", ruby.class_object())?;
    clip_path_circle_class.define_method("position", method!(YClipPathCircle::position, 0))?;
    clip_path_circle_class.define_method("radius", method!(YClipPathCircle::radius, 0))?;

    let clip_path_ellipse_class = clip_path_class.define_class("Ellipse", ruby.class_object())?;
    clip_path_ellipse_class.define_method("position", method!(YClipPathEllipse::position, 0))?;
    clip_path_ellipse_class.define_method("x_radius", method!(YClipPathEllipse::x_radius, 0))?;
    clip_path_ellipse_class.define_method("y_radius", method!(YClipPathEllipse::y_radius, 0))?;

    let clip_path_position_class = clip_path_class.define_class("Position", ruby.class_object())?;
    clip_path_position_class.define_method("x", method!(YClipPathPosition::x, 0))?;
    clip_path_position_class.define_method("y", method!(YClipPathPosition::y, 0))?;

    let _clip_path_position_auto_class = clip_path_class.define_class("PositionAuto", ruby.class_object())?;

    let clip_path_shape_radius_length_class = clip_path_class.define_class("ShapeRadiusLength", ruby.class_object())?;
    clip_path_shape_radius_length_class.define_method("value", method!(YClipPathShapeRadiusLength::value, 0))?;

    let _clip_path_shape_radius_closest_side_class = clip_path_class.define_class("ShapeRadiusClosestSide", ruby.class_object())?;

    let _clip_path_shape_radius_farthest_side_class = clip_path_class.define_class("ShapeRadiusFarthestSide", ruby.class_object())?;

    let clip_path_polygon_class = clip_path_class.define_class("Polygon", ruby.class_object())?;
    clip_path_polygon_class.define_method("fill", method!(YClipPathPolygon::fill, 0))?;
    clip_path_polygon_class.define_method("coordinates", method!(YClipPathPolygon::coordinates, 0))?;

    let clip_path_polygon_coord_class = clip_path_class.define_class("PolygonCoord", ruby.class_object())?;
    clip_path_polygon_coord_class.define_method("x", method!(YClipPathPolygonCoord::x, 0))?;
    clip_path_polygon_coord_class.define_method("y", method!(YClipPathPolygonCoord::y, 0))?;

    let clip_path_path_or_shape_class = clip_path_class.define_class("PathOrShape", ruby.class_object())?;
    clip_path_path_or_shape_class.define_method("value", method!(YClipPathPathOrShape::value, 0))?;

    let clip_path_path_function_class = clip_path_class.define_class("PathFunction", ruby.class_object())?;
    clip_path_path_function_class.define_method("fill", method!(YClipPathPathFunction::fill, 0))?;
    clip_path_path_function_class.define_method("commands", method!(YClipPathPathFunction::commands, 0))?;

    let clip_path_path_command_class = clip_path_class.define_class("PathCommand", ruby.class_object())?;
    clip_path_path_command_class.define_method("value", method!(YClipPathPathCommand::value, 0))?;

    let clip_path_path_command_move_class = clip_path_class.define_class("PathCommandMove", ruby.class_object())?;
    clip_path_path_command_move_class.define_method("point", method!(YClipPathPathCommandMove::point, 0))?;

    let clip_path_path_command_line_class = clip_path_class.define_class("PathCommandLine", ruby.class_object())?;
    clip_path_path_command_line_class.define_method("point", method!(YClipPathPathCommandLine::point, 0))?;

    let clip_path_path_command_hline_class = clip_path_class.define_class("PathCommandHLine", ruby.class_object())?;
    clip_path_path_command_hline_class.define_method("x", method!(YClipPathPathCommandHLine::x, 0))?;

    let clip_path_path_command_vline_class = clip_path_class.define_class("PathCommandVLine", ruby.class_object())?;
    clip_path_path_command_vline_class.define_method("y", method!(YClipPathPathCommandVLine::y, 0))?;

    let clip_path_path_command_cubic_curve_class = clip_path_class.define_class("PathCommandCubicCurve", ruby.class_object())?;
    clip_path_path_command_cubic_curve_class.define_method("control1", method!(YClipPathPathCommandCubicCurve::control1, 0))?;
    clip_path_path_command_cubic_curve_class.define_method("control2", method!(YClipPathPathCommandCubicCurve::control2, 0))?;
    clip_path_path_command_cubic_curve_class.define_method("point", method!(YClipPathPathCommandCubicCurve::point, 0))?;

    let clip_path_path_command_quad_curve_class = clip_path_class.define_class("PathCommandQuadCurve", ruby.class_object())?;
    clip_path_path_command_quad_curve_class.define_method("control1", method!(YClipPathPathCommandQuadCurve::control1, 0))?;
    clip_path_path_command_quad_curve_class.define_method("point", method!(YClipPathPathCommandQuadCurve::point, 0))?;

    let clip_path_path_command_smooth_cubic_class = clip_path_class.define_class("PathCommandSmoothCubic", ruby.class_object())?;
    clip_path_path_command_smooth_cubic_class.define_method("control2", method!(YClipPathPathCommandSmoothCubic::control2, 0))?;
    clip_path_path_command_smooth_cubic_class.define_method("point", method!(YClipPathPathCommandSmoothCubic::point, 0))?;

    let clip_path_path_command_smooth_quad_class = clip_path_class.define_class("PathCommandSmoothQuad", ruby.class_object())?;
    clip_path_path_command_smooth_quad_class.define_method("point", method!(YClipPathPathCommandSmoothQuad::point, 0))?;

    let clip_path_path_command_arc_class = clip_path_class.define_class("PathCommandArc", ruby.class_object())?;
    clip_path_path_command_arc_class.define_method("radii", method!(YClipPathPathCommandArc::radii, 0))?;
    clip_path_path_command_arc_class.define_method("arc_sweep", method!(YClipPathPathCommandArc::arc_sweep, 0))?;
    clip_path_path_command_arc_class.define_method("arc_size", method!(YClipPathPathCommandArc::arc_size, 0))?;
    clip_path_path_command_arc_class.define_method("rotate", method!(YClipPathPathCommandArc::rotate, 0))?;
    clip_path_path_command_arc_class.define_method("point", method!(YClipPathPathCommandArc::point, 0))?;

    let _clip_path_path_command_close_class = clip_path_class.define_class("PathCommandClose", ruby.class_object())?;

    let clip_path_path_command_end_point_to_position_class = clip_path_class.define_class("PathCommandEndPointToPosition", ruby.class_object())?;
    clip_path_path_command_end_point_to_position_class.define_method("x", method!(YClipPathPathCommandEndPointToPosition::x, 0))?;
    clip_path_path_command_end_point_to_position_class.define_method("y", method!(YClipPathPathCommandEndPointToPosition::y, 0))?;

    let clip_path_path_command_end_point_by_coordinate_class = clip_path_class.define_class("PathCommandEndPointByCoordinate", ruby.class_object())?;
    clip_path_path_command_end_point_by_coordinate_class.define_method("coord", method!(YClipPathPathCommandEndPointByCoordinate::coord, 0))?;

    let clip_path_path_coordinate_pair_class = clip_path_class.define_class("PathCoordinatePair", ruby.class_object())?;
    clip_path_path_coordinate_pair_class.define_method("x", method!(YClipPathPathCoordinatePair::x, 0))?;
    clip_path_path_coordinate_pair_class.define_method("y", method!(YClipPathPathCoordinatePair::y, 0))?;

    let clip_path_path_axis_end_point_to_position_class = clip_path_class.define_class("PathAxisEndPointToPosition", ruby.class_object())?;
    clip_path_path_axis_end_point_to_position_class.define_method("value", method!(YClipPathPathAxisEndPointToPosition::value, 0))?;

    let clip_path_path_axis_end_point_by_coordinate_class = clip_path_class.define_class("PathAxisEndPointByCoordinate", ruby.class_object())?;
    clip_path_path_axis_end_point_by_coordinate_class.define_method("value", method!(YClipPathPathAxisEndPointByCoordinate::value, 0))?;

    let clip_path_path_control_point_absolute_class = clip_path_class.define_class("PathControlPointAbsolute", ruby.class_object())?;
    clip_path_path_control_point_absolute_class.define_method("x", method!(YClipPathPathControlPointAbsolute::x, 0))?;
    clip_path_path_control_point_absolute_class.define_method("y", method!(YClipPathPathControlPointAbsolute::y, 0))?;

    let clip_path_path_control_point_relative_class = clip_path_class.define_class("PathControlPointRelative", ruby.class_object())?;
    clip_path_path_control_point_relative_class.define_method("value", method!(YClipPathPathControlPointRelative::value, 0))?;

    let clip_path_path_relative_control_point_class = clip_path_class.define_class("PathRelativeControlPoint", ruby.class_object())?;
    clip_path_path_relative_control_point_class.define_method("coord", method!(YClipPathPathRelativeControlPoint::coord, 0))?;
    clip_path_path_relative_control_point_class.define_method("reference", method!(YClipPathPathRelativeControlPoint::reference, 0))?;

    let clip_path_path_arc_radii_class = clip_path_class.define_class("PathArcRadii", ruby.class_object())?;
    clip_path_path_arc_radii_class.define_method("rx", method!(YClipPathPathArcRadii::rx, 0))?;
    clip_path_path_arc_radii_class.define_method("ry", method!(YClipPathPathArcRadii::ry, 0))?;

    let clip_path_shape_function_class = clip_path_class.define_class("ShapeFunction", ruby.class_object())?;
    clip_path_shape_function_class.define_method("fill", method!(YClipPathShapeFunction::fill, 0))?;
    clip_path_shape_function_class.define_method("commands", method!(YClipPathShapeFunction::commands, 0))?;

    let clip_path_shape_command_class = clip_path_class.define_class("ShapeCommand", ruby.class_object())?;
    clip_path_shape_command_class.define_method("value", method!(YClipPathShapeCommand::value, 0))?;

    let clip_path_shape_command_move_class = clip_path_class.define_class("ShapeCommandMove", ruby.class_object())?;
    clip_path_shape_command_move_class.define_method("point", method!(YClipPathShapeCommandMove::point, 0))?;

    let clip_path_shape_command_line_class = clip_path_class.define_class("ShapeCommandLine", ruby.class_object())?;
    clip_path_shape_command_line_class.define_method("point", method!(YClipPathShapeCommandLine::point, 0))?;

    let clip_path_shape_command_hline_class = clip_path_class.define_class("ShapeCommandHLine", ruby.class_object())?;
    clip_path_shape_command_hline_class.define_method("x", method!(YClipPathShapeCommandHLine::x, 0))?;

    let clip_path_shape_command_vline_class = clip_path_class.define_class("ShapeCommandVLine", ruby.class_object())?;
    clip_path_shape_command_vline_class.define_method("y", method!(YClipPathShapeCommandVLine::y, 0))?;

    let clip_path_shape_command_cubic_curve_class = clip_path_class.define_class("ShapeCommandCubicCurve", ruby.class_object())?;
    clip_path_shape_command_cubic_curve_class.define_method("control1", method!(YClipPathShapeCommandCubicCurve::control1, 0))?;
    clip_path_shape_command_cubic_curve_class.define_method("control2", method!(YClipPathShapeCommandCubicCurve::control2, 0))?;
    clip_path_shape_command_cubic_curve_class.define_method("point", method!(YClipPathShapeCommandCubicCurve::point, 0))?;

    let clip_path_shape_command_quad_curve_class = clip_path_class.define_class("ShapeCommandQuadCurve", ruby.class_object())?;
    clip_path_shape_command_quad_curve_class.define_method("control1", method!(YClipPathShapeCommandQuadCurve::control1, 0))?;
    clip_path_shape_command_quad_curve_class.define_method("point", method!(YClipPathShapeCommandQuadCurve::point, 0))?;

    let clip_path_shape_command_smooth_cubic_class = clip_path_class.define_class("ShapeCommandSmoothCubic", ruby.class_object())?;
    clip_path_shape_command_smooth_cubic_class.define_method("control2", method!(YClipPathShapeCommandSmoothCubic::control2, 0))?;
    clip_path_shape_command_smooth_cubic_class.define_method("point", method!(YClipPathShapeCommandSmoothCubic::point, 0))?;

    let clip_path_shape_command_smooth_quad_class = clip_path_class.define_class("ShapeCommandSmoothQuad", ruby.class_object())?;
    clip_path_shape_command_smooth_quad_class.define_method("point", method!(YClipPathShapeCommandSmoothQuad::point, 0))?;

    let clip_path_shape_command_arc_class = clip_path_class.define_class("ShapeCommandArc", ruby.class_object())?;
    clip_path_shape_command_arc_class.define_method("radii", method!(YClipPathShapeCommandArc::radii, 0))?;
    clip_path_shape_command_arc_class.define_method("arc_sweep", method!(YClipPathShapeCommandArc::arc_sweep, 0))?;
    clip_path_shape_command_arc_class.define_method("arc_size", method!(YClipPathShapeCommandArc::arc_size, 0))?;
    clip_path_shape_command_arc_class.define_method("rotate", method!(YClipPathShapeCommandArc::rotate, 0))?;
    clip_path_shape_command_arc_class.define_method("point", method!(YClipPathShapeCommandArc::point, 0))?;

    let _clip_path_shape_command_close_class = clip_path_class.define_class("ShapeCommandClose", ruby.class_object())?;

    let clip_path_command_end_point_to_position_class = clip_path_class.define_class("CommandEndPointToPosition", ruby.class_object())?;
    clip_path_command_end_point_to_position_class.define_method("horizontal", method!(YClipPathCommandEndPointToPosition::horizontal, 0))?;
    clip_path_command_end_point_to_position_class.define_method("vertical", method!(YClipPathCommandEndPointToPosition::vertical, 0))?;

    let clip_path_command_end_point_by_coordinate_class = clip_path_class.define_class("CommandEndPointByCoordinate", ruby.class_object())?;
    clip_path_command_end_point_by_coordinate_class.define_method("coord", method!(YClipPathCommandEndPointByCoordinate::coord, 0))?;

    let clip_path_coordinate_pair_class = clip_path_class.define_class("CoordinatePair", ruby.class_object())?;
    clip_path_coordinate_pair_class.define_method("x", method!(YClipPathCoordinatePair::x, 0))?;
    clip_path_coordinate_pair_class.define_method("y", method!(YClipPathCoordinatePair::y, 0))?;

    let clip_path_axis_end_point_to_position_class = clip_path_class.define_class("AxisEndPointToPosition", ruby.class_object())?;
    clip_path_axis_end_point_to_position_class.define_method("value", method!(YClipPathAxisEndPointToPosition::value, 0))?;

    let clip_path_axis_end_point_by_coordinate_class = clip_path_class.define_class("AxisEndPointByCoordinate", ruby.class_object())?;
    clip_path_axis_end_point_by_coordinate_class.define_method("value", method!(YClipPathAxisEndPointByCoordinate::value, 0))?;

    let clip_path_control_point_absolute_class = clip_path_class.define_class("ControlPointAbsolute", ruby.class_object())?;
    clip_path_control_point_absolute_class.define_method("value", method!(YClipPathControlPointAbsolute::value, 0))?;

    let clip_path_control_point_relative_class = clip_path_class.define_class("ControlPointRelative", ruby.class_object())?;
    clip_path_control_point_relative_class.define_method("value", method!(YClipPathControlPointRelative::value, 0))?;

    let clip_path_relative_control_point_class = clip_path_class.define_class("RelativeControlPoint", ruby.class_object())?;
    clip_path_relative_control_point_class.define_method("coord", method!(YClipPathRelativeControlPoint::coord, 0))?;
    clip_path_relative_control_point_class.define_method("reference", method!(YClipPathRelativeControlPoint::reference, 0))?;

    let clip_path_arc_radii_class = clip_path_class.define_class("ArcRadii", ruby.class_object())?;
    clip_path_arc_radii_class.define_method("rx", method!(YClipPathArcRadii::rx, 0))?;
    clip_path_arc_radii_class.define_method("ry", method!(YClipPathArcRadii::ry, 0))?;

    let _clip_path_auto_class = clip_path_class.define_class("Auto", ruby.class_object())?;

    let _clip_path_none_class = clip_path_class.define_class("None", ruby.class_object())?;

    let _clip_path_element_dependent_class = clip_path_class.define_class("ElementDependent", ruby.class_object())?;

    let _clip_path_fill_box_class = clip_path_class.define_class("FillBox", ruby.class_object())?;

    let _clip_path_stroke_box_class = clip_path_class.define_class("StrokeBox", ruby.class_object())?;

    let _clip_path_view_box_class = clip_path_class.define_class("ViewBox", ruby.class_object())?;

    let _clip_path_margin_box_class = clip_path_class.define_class("MarginBox", ruby.class_object())?;

    let _clip_path_border_box_class = clip_path_class.define_class("BorderBox", ruby.class_object())?;

    let _clip_path_padding_box_class = clip_path_class.define_class("PaddingBox", ruby.class_object())?;

    let _clip_path_content_box_class = clip_path_class.define_class("ContentBox", ruby.class_object())?;

    Ok(())
}
