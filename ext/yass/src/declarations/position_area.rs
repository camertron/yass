use style::values::computed::PositionArea;

#[magnus::wrap(class = "Yass::Declarations::PositionArea")]
pub struct YPositionArea {
  position_area: PositionArea
}

impl YPositionArea {
  pub fn new(position_area: PositionArea) -> Self {
    Self { position_area }
  }
}
