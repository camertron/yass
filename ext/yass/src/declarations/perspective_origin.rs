use style::values::{generics::position::Position, specified::{PositionComponent, position::{HorizontalPositionKeyword, VerticalPositionKeyword}}};

#[magnus::wrap(class = "Yass::Declarations::PerspectiveOrigin")]
pub struct YPerspectiveOrigin {
  specified_value: Box<Position<PositionComponent<HorizontalPositionKeyword>, PositionComponent<VerticalPositionKeyword>>>
}

impl YPerspectiveOrigin {
  pub fn new(specified_value: Box<Position<PositionComponent<HorizontalPositionKeyword>, PositionComponent<VerticalPositionKeyword>>>) -> Self {
    Self { specified_value }
  }
}
