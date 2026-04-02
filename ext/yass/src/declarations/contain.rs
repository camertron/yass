use magnus::{Error, RArray, Ruby, typed_data};
use style::values::computed::Contain;

#[magnus::wrap(class = "Yass::Declarations::Contain")]
pub struct YContain {
    contain: Contain
}

impl YContain {
    pub fn new(contain: Contain) -> Self {
        Self { contain }
    }

    pub fn values(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        let contain = rb_self.contain;
        let result = ruby.ary_new();

        if contain.is_empty() {
            result.push(ruby.intern("none"))?;
            return Ok(result);
        }

        if contain.contains(Contain::STRICT) {
            result.push(ruby.intern("strict"))?;
            return Ok(result);
        }

        if contain.contains(Contain::CONTENT) {
            result.push(ruby.intern("content"))?;
        }

        if contain.contains(Contain::SIZE) {
            result.push(ruby.intern("size"))?;
        } else {
            if contain.contains(Contain::INLINE_SIZE) {
                result.push(ruby.intern("inline_size"))?;
            }

            if contain.contains(Contain::BLOCK_SIZE) {
                result.push(ruby.intern("block_size"))?;
            }
        }

        if !contain.contains(Contain::CONTENT) {
            if contain.contains(Contain::LAYOUT) {
                result.push(ruby.intern("layout"))?;
            }

            if contain.contains(Contain::STYLE) {
                result.push(ruby.intern("style"))?;
            }

            if contain.contains(Contain::PAINT) {
                result.push(ruby.intern("paint"))?;
            }
        }

        Ok(result)
    }

    pub fn is_none(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.contain.is_empty()
    }

    pub fn is_inline_size(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.contain.contains(Contain::STRICT) ||
            rb_self.contain.contains(Contain::SIZE) ||
            rb_self.contain.contains(Contain::INLINE_SIZE)
    }

    pub fn is_block_size(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.contain.contains(Contain::STRICT) ||
            rb_self.contain.contains(Contain::SIZE) ||
            rb_self.contain.contains(Contain::BLOCK_SIZE)
    }

    pub fn is_layout(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.contain.contains(Contain::STRICT) ||
            rb_self.contain.contains(Contain::CONTENT) ||
            rb_self.contain.contains(Contain::LAYOUT)
    }

    pub fn is_style(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.contain.contains(Contain::STRICT) ||
            rb_self.contain.contains(Contain::CONTENT) ||
            rb_self.contain.contains(Contain::STYLE)
    }

    pub fn is_paint(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.contain.contains(Contain::STRICT) ||
            rb_self.contain.contains(Contain::CONTENT) ||
            rb_self.contain.contains(Contain::PAINT)
    }

    pub fn is_size(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.contain.contains(Contain::STRICT) || rb_self.contain.contains(Contain::SIZE)
    }

    pub fn is_content(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.contain.contains(Contain::STRICT) || rb_self.contain.contains(Contain::CONTENT)
    }

    pub fn is_strict(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.contain.contains(Contain::STRICT)
    }
}
