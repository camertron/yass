use style::computed_values::text_decoration_style::T;

#[magnus::wrap(class = "Yass::Declarations::TextDecorationStyle")]
pub struct YTextDecorationStyle {
  specified_value: T
}

impl YTextDecorationStyle {
  pub fn new(specified_value: T) -> Self {
    Self { specified_value }
  }
}
