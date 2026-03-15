use style::values::computed::ContentDistribution;

#[magnus::wrap(class = "Yass::Declarations::JustifyContent")]
pub struct YJustifyContent {
  content_distribution: ContentDistribution
}

impl YJustifyContent {
  pub fn new(content_distribution: ContentDistribution) -> Self {
    Self { content_distribution }
  }
}
