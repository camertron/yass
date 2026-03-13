use magnus::{Error, Module, RModule, Ruby, Value, value::ReprValue};

pub fn singleton_instance(class_name: &str, ruby: &Ruby) -> Result<Value, Error> {
    let class = ruby
        .class_object()
        .const_get::<_, RModule>(class_name)?;

    Ok(class.funcall_public::<&str, [Value; 0], Value>("instance", [])?)
}
