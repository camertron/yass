use style::values::computed::PositionProperty;

#[magnus::wrap(class = "Yass::Declarations::Position")]
pub struct YPosition {
  position_property: PositionProperty
}

impl YPosition {
  pub fn new(position_property: PositionProperty) -> Self {
    Self { position_property }
  }
}
