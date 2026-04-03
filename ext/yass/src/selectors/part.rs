use magnus::{Error, RArray, Ruby};

#[magnus::wrap(class = "Yass::Selector::Part")]
pub struct YPart {
    items: Vec<String>
}

impl YPart {
    pub fn new(items: Vec<String>) -> Self {
        Self { items }
    }

    pub fn items(&self, ruby: &Ruby) -> Result<RArray, Error> {
        let result = ruby.ary_new_capa(self.items.len());

        for item in &self.items {
            result.push(ruby.str_new(item))?;
        }

        Ok(result)
    }
}
