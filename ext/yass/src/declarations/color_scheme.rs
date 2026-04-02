use magnus::{Error, RArray, Ruby, typed_data};
use style::values::{computed::ColorScheme};
use style_traits::ToCss;

#[magnus::wrap(class = "Yass::Declarations::ColorScheme")]
pub struct YColorScheme {
    color_scheme: ColorScheme
}

impl YColorScheme {
    pub fn new(color_scheme: ColorScheme) -> Self {
        Self { color_scheme }
    }

    pub fn values(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        let css = rb_self.color_scheme.to_css_string();
        let idents: Vec<&str> = css.split_whitespace().collect();
        let result = ruby.ary_new_capa(idents.len());

        for ident in idents {
            result.push(ruby.str_new(ident))?
        }

        Ok(result)
    }
}
