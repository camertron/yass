use magnus::{RString, Ruby, typed_data, value::Id};
use ::style::values::computed::font::{FamilyName, FontFamilyNameSyntax};

pub mod init;
pub mod metrics;
pub mod source;
pub mod stretch;
pub mod style;
pub mod weight;

#[magnus::wrap(class = "Yass::UnicodeRange")]
pub struct YUnicodeRange {
    start: u32,
    end: u32,
}

impl YUnicodeRange {
    pub fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    pub fn start(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> u32 {
        rb_self.start
    }

    pub fn end(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> u32 {
        rb_self.end
    }
}

#[magnus::wrap(class = "Yass::Font::FamilyName")]
pub struct YFontFamilyName {
    name: String,
    syntax: FontFamilyNameSyntax,
}

impl YFontFamilyName {
    pub fn new(family: FamilyName) -> Self {
        Self {
            name: family.name.to_string(),
            syntax: family.syntax,
        }
    }

    pub fn name(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> RString {
        ruby.str_new(&rb_self.name)
    }

    pub fn syntax(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.syntax {
            FontFamilyNameSyntax::Quoted => ruby.intern("quoted"),
            FontFamilyNameSyntax::Identifiers => ruby.intern("identifiers"),
        }
    }
}

#[magnus::wrap(class = "Yass::SystemFont")]
pub struct YSystemFont {}

impl YSystemFont {
    pub fn new() -> Self {
        Self {}
    }
}
