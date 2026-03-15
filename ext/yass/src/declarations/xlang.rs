use style::values::computed::XLang;

#[magnus::wrap(class = "Yass::Declarations::XLang")]
pub struct YXLang {
  xlang: XLang
}

impl YXLang {
  pub fn new(xlang: XLang) -> Self {
    Self { xlang }
  }
}
