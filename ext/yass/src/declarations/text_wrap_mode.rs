use style::computed_values::text_wrap_mode::T;

#[magnus::wrap(class = "Yass::Declarations::TextWrapMode")]
pub struct YTextWrapMode {
  specified_value: T
}

impl YTextWrapMode {
  pub fn new(specified_value: T) -> Self {
    Self { specified_value }
  }
}
