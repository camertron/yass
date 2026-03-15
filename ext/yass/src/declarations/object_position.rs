use style::values::{generics::position::Position, specified::{PositionComponent, position::{HorizontalPositionKeyword, VerticalPositionKeyword}}};

#[magnus::wrap(class = "Yass::Declarations::ObjectPosition")]
pub struct YObjectPosition {
  specified_value: Box<Position<PositionComponent<HorizontalPositionKeyword>, PositionComponent<VerticalPositionKeyword>>>
}

impl YObjectPosition {
  pub fn new(specified_value: Box<Position<PositionComponent<HorizontalPositionKeyword>, PositionComponent<VerticalPositionKeyword>>>) -> Self {
    Self { specified_value }
  }
}
