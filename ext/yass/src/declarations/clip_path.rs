use style::values::{
  CssUrl,

  generics::{
    basic_shape::{
      BasicShape, ClipPath as StyloClipPath
    },

    position::Position
  },

  specified::{
    Angle,
    LengthPercentage,
    PositionComponent,
    basic_shape::BasicShapeRect,
    position::{HorizontalPositionKeyword, VerticalPositionKeyword}
  }
};

type ClipPath = StyloClipPath<
  BasicShape<
    Angle,

    Position<
      PositionComponent<HorizontalPositionKeyword>,
      PositionComponent<VerticalPositionKeyword>
    >,

    LengthPercentage,
    BasicShapeRect
  >,

  CssUrl
>;

#[magnus::wrap(class = "Yass::Declarations::ClipPath")]
pub struct YClipPath {
  clip_path: ClipPath
}

impl YClipPath {
  pub fn new(clip_path: ClipPath) -> Self {
    Self { clip_path }
  }
}
