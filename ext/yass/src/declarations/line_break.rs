use style::values::computed::LineBreak;

#[magnus::wrap(class = "Yass::Declarations::LineBreak")]
pub struct YLineBreak {
  line_break: LineBreak
}

impl YLineBreak {
  pub fn new(line_break: LineBreak) -> Self {
    Self { line_break }
  }
}
