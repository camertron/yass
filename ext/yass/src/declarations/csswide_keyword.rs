use style::properties::WideKeywordDeclaration;

#[magnus::wrap(class = "Yass::Declarations::CSSWideKeyword")]
pub struct YCSSWideKeyword {
  wide_keyword_declaration: WideKeywordDeclaration
}

impl YCSSWideKeyword {
  pub fn new(wide_keyword_declaration: WideKeywordDeclaration) -> Self {
    Self { wide_keyword_declaration }
  }
}
