use magnus::{Error, RArray, Ruby, typed_data};
use style::values::computed::TextDecorationLine;

#[magnus::wrap(class = "Yass::Declarations::TextDecorationLine")]
pub struct YTextDecorationLine {
    text_decoration_line: TextDecorationLine
}

impl YTextDecorationLine {
    pub fn new(text_decoration_line: TextDecorationLine) -> Self {
        Self { text_decoration_line }
    }

    pub fn values(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        let text_decoration_line = rb_self.text_decoration_line;
        let result = ruby.ary_new();

        if text_decoration_line.is_empty() {
            result.push(ruby.intern("none"))?;
            return Ok(result);
        }

        if text_decoration_line.contains(TextDecorationLine::UNDERLINE) {
            result.push(ruby.intern("underline"))?;
        }

        if text_decoration_line.contains(TextDecorationLine::OVERLINE) {
            result.push(ruby.intern("overline"))?;
        }

        if text_decoration_line.contains(TextDecorationLine::LINE_THROUGH) {
            result.push(ruby.intern("line_through"))?;
        }

        if text_decoration_line.contains(TextDecorationLine::BLINK) {
            result.push(ruby.intern("blink"))?;
        }

        if text_decoration_line.contains(TextDecorationLine::SPELLING_ERROR) {
            result.push(ruby.intern("spelling_error"))?;
        }

        if text_decoration_line.contains(TextDecorationLine::GRAMMAR_ERROR) {
            result.push(ruby.intern("grammar_error"))?;
        }

        Ok(result)
    }

    pub fn is_none(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.text_decoration_line.is_empty()
    }

    pub fn is_underline(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.text_decoration_line.contains(TextDecorationLine::UNDERLINE)
    }

    pub fn is_overline(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.text_decoration_line.contains(TextDecorationLine::OVERLINE)
    }

    pub fn is_line_through(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.text_decoration_line.contains(TextDecorationLine::LINE_THROUGH)
    }

    pub fn is_blink(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.text_decoration_line.contains(TextDecorationLine::BLINK)
    }

    pub fn is_spelling_error(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.text_decoration_line.contains(TextDecorationLine::SPELLING_ERROR)
    }

    pub fn is_grammar_error(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.text_decoration_line.contains(TextDecorationLine::GRAMMAR_ERROR)
    }
}
