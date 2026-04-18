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
