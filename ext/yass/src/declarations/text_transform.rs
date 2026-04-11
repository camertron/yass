use magnus::{Error, RArray, Ruby, typed_data};
use style::values::computed::TextTransform;
use style::values::specified::text::TextTransformCase;

#[magnus::wrap(class = "Yass::Declarations::TextTransform")]
pub struct YTextTransform {
    text_transform: TextTransform
}

impl YTextTransform {
    pub fn new(text_transform: TextTransform) -> Self {
        Self { text_transform }
    }

    pub fn values(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        let text_transform = rb_self.text_transform;
        let result = ruby.ary_new();

        if text_transform.is_none() {
            result.push(ruby.intern("none"))?;
            return Ok(result);
        }

        match text_transform.case() {
            TextTransformCase::Uppercase => {
                result.push(ruby.intern("uppercase"))?;
            },
            TextTransformCase::Lowercase => {
                result.push(ruby.intern("lowercase"))?;
            },
            TextTransformCase::Capitalize => {
                result.push(ruby.intern("capitalize"))?;
            },
            TextTransformCase::None => {
                // No case transform, continue
            },
        }

        if text_transform.contains(TextTransform::FULL_WIDTH) {
            result.push(ruby.intern("full_width"))?;
        }

        if text_transform.contains(TextTransform::FULL_SIZE_KANA) {
            result.push(ruby.intern("full_size_kana"))?;
        }

        Ok(result)
    }
}

