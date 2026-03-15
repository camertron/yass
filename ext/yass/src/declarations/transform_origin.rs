use style::values::{generics::transform::TransformOrigin, specified::{Length, position::{HorizontalPositionKeyword, VerticalPositionKeyword}, transform::OriginComponent}};

#[magnus::wrap(class = "Yass::Declarations::TransformOrigin")]
pub struct YTransformOrigin {
  specified_value: Box<TransformOrigin<OriginComponent<HorizontalPositionKeyword>, OriginComponent<VerticalPositionKeyword>, Length>>
}

impl YTransformOrigin {
  pub fn new(specified_value: Box<TransformOrigin<OriginComponent<HorizontalPositionKeyword>, OriginComponent<VerticalPositionKeyword>, Length>>) -> Self {
    Self { specified_value }
  }
}
