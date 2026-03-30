use magnus::{Error, RArray, Ruby, typed_data};
use style::properties::longhands::background_origin::{SpecifiedValue, single_value::SpecifiedValue as SingleSpecifiedValue};

#[magnus::wrap(class = "Yass::Declarations::BackgroundOrigin")]
pub struct YBackgroundOrigin {
    specified_value: SpecifiedValue
}

impl YBackgroundOrigin {
    pub fn new(specified_value: SpecifiedValue) -> Self {
        Self { specified_value }
    }

    pub fn values(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        let result = ruby.ary_new_capa(rb_self.specified_value.0.len());

        for origin in rb_self.specified_value.0.as_ref() {
            let id = match origin {
                SingleSpecifiedValue::PaddingBox => ruby.intern("padding_box"),
                SingleSpecifiedValue::BorderBox => ruby.intern("border_box"),
                SingleSpecifiedValue::ContentBox => ruby.intern("content_box"),
            };

            result.push(id)?;
        }

        Ok(result)
    }
}
