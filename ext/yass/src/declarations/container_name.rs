use magnus::{Error, RArray, Ruby, typed_data};
use style::values::computed::ContainerName;
use style_traits::ToCss;

#[magnus::wrap(class = "Yass::Declarations::ContainerName")]
pub struct YContainerName {
    container_name: ContainerName
}

impl YContainerName {
    pub fn new(container_name: ContainerName) -> Self {
        Self { container_name }
    }

    pub fn values(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        // Stylo keeps the underlying list private, so we split the serialized
        // CSS value to expose each container name.
        let css = rb_self.container_name.to_css_string();
        let idents: Vec<&str> = css.split_whitespace().collect();
        let result = ruby.ary_new_capa(idents.len());

        for ident in idents {
            result.push(ruby.str_new(ident))?;
        }

        Ok(result)
    }

    pub fn is_none(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.container_name.is_none()
    }
}
