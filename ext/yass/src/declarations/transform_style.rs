use style::values::computed::TransformStyle;

#[magnus::wrap(class = "Yass::Declarations::TransformStyle")]
pub struct YTransformStyle {
  transform_style: TransformStyle
}

impl YTransformStyle {
  pub fn new(transform_style: TransformStyle) -> Self {
    Self { transform_style }
  }
}
