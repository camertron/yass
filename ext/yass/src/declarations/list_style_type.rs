use style::values::computed::ListStyleType;

#[magnus::wrap(class = "Yass::Declarations::ListStyleType")]
pub struct YListStyleType {
  list_style_type: ListStyleType
}

impl YListStyleType {
  pub fn new(list_style_type: ListStyleType) -> Self {
    Self { list_style_type }
  }
}
