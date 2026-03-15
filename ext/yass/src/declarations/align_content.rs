use style::values::computed::ContentDistribution;

#[magnus::wrap(class = "Yass::Declarations::AlignContent")]
pub struct YAlignContent {
  content_distribution: ContentDistribution
}

impl YAlignContent {
  pub fn new(content_distribution: ContentDistribution) -> Self {
    Self { content_distribution }
  }
}
