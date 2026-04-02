use magnus::{Error, RArray, Ruby, typed_data};
use style::values::computed::ContainerType;

#[magnus::wrap(class = "Yass::Declarations::ContainerType")]
pub struct YContainerType {
    container_type: ContainerType
}

impl YContainerType {
    pub fn new(container_type: ContainerType) -> Self {
        Self { container_type }
    }

    pub fn values(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        let container_type = rb_self.container_type;
        let result = ruby.ary_new();

        if container_type.is_normal() {
            result.push(ruby.intern("normal"))?;
            return Ok(result);
        }

        if container_type.contains(ContainerType::SIZE) {
            result.push(ruby.intern("size"))?;
        } else if container_type.contains(ContainerType::INLINE_SIZE) {
            result.push(ruby.intern("inline_size"))?;
        }

        if container_type.contains(ContainerType::SCROLL_STATE) {
            result.push(ruby.intern("scroll_state"))?;
        }

        Ok(result)
    }

    pub fn is_normal(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.container_type.is_normal()
    }

    pub fn is_inline_size(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.container_type.contains(ContainerType::SIZE) ||
            rb_self.container_type.contains(ContainerType::INLINE_SIZE)
    }

    pub fn is_size(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.container_type.contains(ContainerType::SIZE)
    }

    pub fn is_scroll_state(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.container_type.contains(ContainerType::SCROLL_STATE)
    }
}
