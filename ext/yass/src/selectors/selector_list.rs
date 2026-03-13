use magnus::{Error, RArray, Ruby, gc, typed_data, value::Opaque};
use selectors::parser::Selector;
use style::selector_parser::SelectorImpl;

use crate::selectors::YSelector;

pub struct YSelectorList {
    pub selectors: Vec<Opaque<typed_data::Obj<YSelector>>>
}

impl YSelectorList {
    pub fn new(selector_list: &[Selector<SelectorImpl>], ruby: &Ruby) -> Self {
        let mut new_selectors: Vec<Opaque<typed_data::Obj<YSelector>>> = vec![];

        for selector in selector_list {
            let yselector = YSelector::new(selector.clone());
            new_selectors.push(ruby.obj_wrap(yselector).into());
        }

        Self { selectors: new_selectors }
    }

    pub fn to_a(&self, ruby: &Ruby) -> Result<RArray, Error> {
        let result = ruby.ary_new_capa(self.selectors.len());

        for selector in &self.selectors {
            result.push(ruby.get_inner(*selector))?;
        }

        Ok(result)
    }

    pub fn mark(&self, marker: &gc::Marker) {
        marker.mark_slice(self.selectors.as_slice());
    }
}
