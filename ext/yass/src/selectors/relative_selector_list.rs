use magnus::{Error, RArray, Ruby, gc, typed_data, value::Opaque};
use selectors::parser::RelativeSelector;
use style::selector_parser::SelectorImpl;

use crate::selectors::YRelativeSelector;

pub struct YRelativeSelectorList {
    pub relative_selectors: Vec<Opaque<typed_data::Obj<YRelativeSelector>>>
}

impl YRelativeSelectorList {
    pub fn new(selector_list: &[RelativeSelector<SelectorImpl>], ruby: &Ruby) -> Self {
        let mut new_selectors: Vec<Opaque<typed_data::Obj<YRelativeSelector>>> = vec![];

        for relative_selector in selector_list {
            let yselector = YRelativeSelector::new(relative_selector.clone(), ruby);
            new_selectors.push(Opaque::from(ruby.obj_wrap(yselector)));
        }

        Self { relative_selectors: new_selectors }
    }

    pub fn to_a(&self, ruby: &Ruby) -> Result<RArray, Error> {
        let result = ruby.ary_new_capa(self.relative_selectors.len());

        for selector in &self.relative_selectors {
            result.push(ruby.get_inner(*selector))?;
        }

        Ok(result)
    }

    pub fn mark(&self, marker: &gc::Marker) {
        marker.mark_slice(self.relative_selectors.as_slice());
    }
}
