use style::computed_values::_webkit_text_security::T;

#[magnus::wrap(class = "Yass::Declarations::WebkitTextSecurity")]
pub struct YWebkitTextSecurity {
  specified_value: T
}

impl YWebkitTextSecurity {
  pub fn new(specified_value: T) -> Self {
    Self { specified_value }
  }
}
