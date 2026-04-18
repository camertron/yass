use magnus::{Error, Module, RModule, Ruby, method};

use crate::general::YUnicodeRange;

pub fn init(ruby: &Ruby, yass_module: &RModule) -> Result<(), Error> {
    let unicode_range_class = yass_module.define_class("UnicodeRange", ruby.class_object())?;
    unicode_range_class.define_method("start", method!(YUnicodeRange::start, 0))?;
    unicode_range_class.define_method("end", method!(YUnicodeRange::end, 0))?;

    Ok(())
}
