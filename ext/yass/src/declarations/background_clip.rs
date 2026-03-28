use magnus::{Error, RArray, Ruby, typed_data};
use style::{OwnedSlice, computed_values::background_clip::SingleComputedValue};
use style_traits::ToCss;

#[magnus::wrap(class = "Yass::Declarations::BackgroundClip")]
pub struct YBackgroundClip {
    specified_value: OwnedSlice<SingleComputedValue>
}

impl YBackgroundClip {
    pub fn new(specified_value: OwnedSlice<SingleComputedValue>) -> Self {
        Self { specified_value }
    }

    pub fn values(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        let result = ruby.ary_new_capa(rb_self.specified_value.len());

        for clip in rb_self.specified_value.as_ref() {
            let css = clip.to_css_string();
            result.push(ruby.intern(&css))?;
        }

        Ok(result)
    }
}
