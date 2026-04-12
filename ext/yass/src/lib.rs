mod cached_value;
mod declarations;
mod rules;
mod selectors;
mod sheet;
mod utils;
mod cached_value_list;

use magnus::{Error, Ruby, function, method, prelude::*};
use style::context::QuirksMode;
use style::media_queries::MediaList;
use style::shared_lock::SharedRwLock;
use style::servo_arc::Arc;
use style::stylesheets::{AllowImportRules, Origin, Stylesheet, UrlExtraData};
use url::Url;

use crate::sheet::YSheet;

fn parse(css: String) -> YSheet {
    let lock = SharedRwLock::new();
    let base_url = Url::parse("https://example.com/style.css").expect("invalid URL");
    let url_data = UrlExtraData::from(base_url);

    // `MediaList::empty()` means "applies to all media".
    let media = Arc::new(lock.wrap(MediaList::empty()));

    let stylesheet = Stylesheet::from_str(
        &css,
        url_data,
        Origin::Author,             // stylesheet origin: Author / User / UserAgent
        media,
        lock.clone(),
        None,     // no external @import loader
        None,        // no error reporter
        QuirksMode::NoQuirks,
        AllowImportRules::Yes,
    );

    YSheet::new(stylesheet)
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    <bool as stylo_static_prefs::Preference>::set("layout.unimplemented", true);
    <bool as stylo_static_prefs::Preference>::set(
        "layout.css.backdrop-filter.enabled",
        true,
    );
    <bool as stylo_static_prefs::Preference>::set("layout.variable_fonts.enabled", true);
    <bool as stylo_static_prefs::Preference>::set("layout.css.font-variations.enabled", true);
    <bool as stylo_static_prefs::Preference>::set("layout.container-queries.enabled", true);
    <bool as stylo_static_prefs::Preference>::set("layout.columns.enabled", true);
    <bool as stylo_static_prefs::Preference>::set("layout.grid.enabled", true);
    <bool as stylo_static_prefs::Preference>::set("layout.writing-mode.enabled", true);

    let yass_module = ruby.define_module("Yass")?;

    let parser_module = yass_module.define_module("Parser")?;
    parser_module.define_singleton_method("parse", function!(parse, 1))?;

    let stylesheet_class = yass_module.define_class("Stylesheet", ruby.class_object())?;
    stylesheet_class.define_method("rules", method!(YSheet::rules, 0))?;

    declarations::init::init(ruby, &yass_module)?;
    rules::init::init(ruby, &yass_module)?;
    selectors::init::init(ruby, &yass_module)?;

    Ok(())
}
