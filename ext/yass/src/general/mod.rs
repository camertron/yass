use magnus::{Ruby, typed_data};

pub mod init;

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

#[magnus::wrap(class = "Yass::SourceLocation")]
pub struct YSourceLocation {
    line: u32,
    column: u32,
}

impl YSourceLocation {
    pub fn new(line: u32, column: u32) -> Self {
        Self { line, column }
    }

    pub fn line(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> u32 {
        rb_self.line
    }

    pub fn column(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> u32 {
        rb_self.column
    }
}
