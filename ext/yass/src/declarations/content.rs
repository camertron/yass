use style::values::specified::Content;

#[magnus::wrap(class = "Yass::Declarations::Content")]
pub struct YContent {
  content: Content
}

impl YContent {
  pub fn new(content: Content) -> Self {
    Self { content }
  }
}
