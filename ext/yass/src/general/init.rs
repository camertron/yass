use magnus::{Error, Module, RModule, Ruby, method};

use crate::general::{YSourceLocation, YUnicodeRange};

pub fn init(ruby: &Ruby, yass_module: &RModule) -> Result<(), Error> {
    let unicode_range_class = yass_module.define_class("UnicodeRange", ruby.class_object())?;
    unicode_range_class.define_method("start", method!(YUnicodeRange::start, 0))?;
    unicode_range_class.define_method("end", method!(YUnicodeRange::end, 0))?;

    let source_location_class = yass_module.define_class("SourceLocation", ruby.class_object())?;
    source_location_class.define_method("line", method!(YSourceLocation::line, 0))?;
    source_location_class.define_method("column", method!(YSourceLocation::column, 0))?;

    Ok(())
}
