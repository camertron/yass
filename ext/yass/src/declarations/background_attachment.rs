use magnus::{Error, RArray, Ruby, typed_data};
use style::properties::longhands::background_attachment::SpecifiedValue;
use style_traits::ToCss;

#[magnus::wrap(class = "Yass::Declarations::BackgroundAttachment")]
pub struct YBackgroundAttachment {
    specified_value: SpecifiedValue
}

impl YBackgroundAttachment {
    pub fn new(specified_value: SpecifiedValue) -> Self {
        Self { specified_value }
    }

    pub fn values(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        let result = ruby.ary_new_capa(rb_self.specified_value.0.len());

        for attachment in rb_self.specified_value.0.as_ref() {
            let css = attachment.to_css_string();
            result.push(ruby.intern(&css))?;
        }

        Ok(result)
    }
}
