use style::computed_values::white_space_collapse::T;

#[magnus::wrap(class = "Yass::Declarations::WhiteSpaceCollapse")]
pub struct YWhiteSpaceCollapse {
  specified_value: T
}

impl YWhiteSpaceCollapse {
  pub fn new(specified_value: T) -> Self {
    Self { specified_value }
  }
}
