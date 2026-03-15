use style::values::computed::TextDecorationLine;

#[magnus::wrap(class = "Yass::Declarations::TextDecorationLine")]
pub struct YTextDecorationLine {
  text_decoration_line: TextDecorationLine
}

impl YTextDecorationLine {
  pub fn new(text_decoration_line: TextDecorationLine) -> Self {
    Self { text_decoration_line }
  }
}
