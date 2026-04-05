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

use crate::rules::YRule;
use crate::selectors::{YAnPlusB, YRelativeSelector, YSelector, YSelectorChild, YSpecificNamespaceConstraint};
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

    let yass_module = ruby.define_module("Yass")?;

    let parser_module = yass_module.define_module("Parser")?;
    parser_module.define_singleton_method("parse", function!(parse, 1))?;

    let stylesheet_class = yass_module.define_class("Stylesheet", ruby.class_object())?;
    stylesheet_class.define_method("rules", method!(YSheet::rules, 0))?;

    let rule_class = yass_module.define_class("Rule", ruby.class_object())?;
    let style_rule_class = yass_module.define_class("StyleRule", rule_class)?;
    style_rule_class.define_method("selectors", method!(YRule::selectors, 0))?;
    style_rule_class.define_method("declarations", method!(YRule::declarations, 0))?;

    let _media_rule_class = yass_module.define_class("MediaRule", rule_class)?;

    let _unimplemented_rule_class = yass_module.define_class("UnimplementedRule", rule_class)?;

    let selector_class = yass_module.define_class("Selector", ruby.class_object())?;
    selector_class.define_method("children", method!(YSelector::children, 0))?;

    let relative_selector_class = yass_module.define_class("RelativeSelector", ruby.class_object())?;
    relative_selector_class.define_method("selector", method!(YRelativeSelector::selector, 0))?;
    relative_selector_class.define_method("match_hint", method!(YRelativeSelector::match_hint, 0))?;

    let selector_child_class = yass_module.define_class("SelectorChild", ruby.class_object())?;

    let combinator_class = selector_class.define_class("Combinator", selector_child_class)?;
    selector_child_class.define_method("kind", method!(YSelectorChild::kind, 0))?;
    combinator_class.define_method("ancestor?", method!(YSelectorChild::is_ancestor, 0))?;
    combinator_class.define_method("pseudo_element?", method!(YSelectorChild::is_pseudo_element, 0))?;
    combinator_class.define_method("sibling?", method!(YSelectorChild::is_sibling, 0))?;

    let local_name_class = selector_class.define_class("LocalName", selector_child_class)?;
    local_name_class.define_method("kind", method!(YSelectorChild::kind, 0))?;
    local_name_class.define_method("value", method!(YSelectorChild::value, 0))?;

    let id_class = selector_class.define_class("Id", selector_child_class)?;
    id_class.define_method("kind", method!(YSelectorChild::kind, 0))?;
    id_class.define_method("value", method!(YSelectorChild::value, 0))?;

    let class_class = selector_class.define_class("Klass", selector_child_class)?;
    class_class.define_method("kind", method!(YSelectorChild::kind, 0))?;
    class_class.define_method("value", method!(YSelectorChild::value, 0))?;

    let negation_class = selector_class.define_class("Negation", selector_child_class)?;
    negation_class.define_method("kind", method!(YSelectorChild::kind, 0))?;
    negation_class.define_method("selectors", method!(YSelectorChild::selectors, 0))?;

    let attribute_in_no_namespace_exists_class = selector_class.define_class("AttributeInNoNamespaceExists", selector_child_class)?;
    attribute_in_no_namespace_exists_class.define_method("kind", method!(YSelectorChild::kind, 0))?;

    let attribute_in_no_namespace_class = selector_class.define_class("AttributeInNoNamespace", selector_child_class)?;
    attribute_in_no_namespace_class.define_method("kind", method!(YSelectorChild::kind, 0))?;
    attribute_in_no_namespace_class.define_method("name", method!(YSelectorChild::name, 0))?;
    attribute_in_no_namespace_class.define_method("value", method!(YSelectorChild::value, 0))?;
    attribute_in_no_namespace_class.define_method("operator", method!(YSelectorChild::operator, 0))?;
    attribute_in_no_namespace_class.define_method("case_sensitivity", method!(YSelectorChild::case_sensitivity, 0))?;

    let attribute_other_class = selector_class.define_class("AttributeOther", selector_child_class)?;
    attribute_other_class.define_method("kind", method!(YSelectorChild::kind, 0))?;
    attribute_other_class.define_method("namespace", method!(YSelectorChild::namespace, 0))?;

    let any_namespace_class = selector_class.define_class("AnyNamespace", ruby.class_object())?;
    let _ = ruby.require("singleton");
    any_namespace_class.include_module(ruby.class_object().const_get("Singleton")?)?;

    let specific_namespace_constraint_class = selector_class.define_class("SpecificNamespaceConstraint", ruby.class_object())?;
    specific_namespace_constraint_class.define_method("prefix", method!(YSpecificNamespaceConstraint::prefix, 0))?;
    specific_namespace_constraint_class.define_method("url", method!(YSpecificNamespaceConstraint::url, 0))?;

    let explicit_universal_type_class = selector_class.define_class("ExplicitUniversalType", selector_child_class)?;
    explicit_universal_type_class.define_method("kind", method!(YSelectorChild::kind, 0))?;

    let explicit_any_namespace_class = selector_class.define_class("ExplicitAnyNamespace", selector_child_class)?;
    explicit_any_namespace_class.define_method("kind", method!(YSelectorChild::kind, 0))?;

    let explicit_no_namespace_class = selector_class.define_class("ExplicitNoNamespace", selector_child_class)?;
    explicit_no_namespace_class.define_method("kind", method!(YSelectorChild::kind, 0))?;

    let default_namespace_class = selector_class.define_class("DefaultNamespace", selector_child_class)?;
    default_namespace_class.define_method("kind", method!(YSelectorChild::kind, 0))?;
    default_namespace_class.define_method("url", method!(YSelectorChild::url, 0))?;

    let namespace_class = selector_class.define_class("Namespace", selector_child_class)?;
    namespace_class.define_method("kind", method!(YSelectorChild::kind, 0))?;
    namespace_class.define_method("prefix", method!(YSelectorChild::prefix, 0))?;
    namespace_class.define_method("url", method!(YSelectorChild::url, 0))?;

    let root_class = selector_class.define_class("Root", selector_child_class)?;
    root_class.define_method("kind", method!(YSelectorChild::kind, 0))?;

    let empty_class = selector_class.define_class("Empty", selector_child_class)?;
    empty_class.define_method("kind", method!(YSelectorChild::kind, 0))?;

    let scope_class = selector_class.define_class("Scope", selector_child_class)?;
    scope_class.define_method("kind", method!(YSelectorChild::kind, 0))?;

    let implicit_scope_class = selector_class.define_class("ImplicitScope", selector_child_class)?;
    implicit_scope_class.define_method("kind", method!(YSelectorChild::kind, 0))?;

    let parent_selector_class = selector_class.define_class("ParentSelector", selector_child_class)?;
    parent_selector_class.define_method("kind", method!(YSelectorChild::kind, 0))?;

    let nth_class = selector_class.define_class("Nth", selector_child_class)?;
    nth_class.define_method("kind", method!(YSelectorChild::kind, 0))?;
    nth_class.define_method("type", method!(YSelectorChild::ty, 0))?;
    nth_class.define_method("function?", method!(YSelectorChild::is_method, 0))?;
    nth_class.define_method("an_plus_b", method!(YSelectorChild::an_plus_b, 0))?;

    let an_plus_b_class = selector_class.define_class("AnPlusB", ruby.class_object())?;
    an_plus_b_class.define_method("a", method!(YAnPlusB::a, 0))?;
    an_plus_b_class.define_method("b", method!(YAnPlusB::b, 0))?;

    let nth_of_class = selector_class.define_class("NthOf", selector_child_class)?;
    nth_of_class.define_method("kind", method!(YSelectorChild::kind, 0))?;
    nth_of_class.define_method("nth", method!(YSelectorChild::nth, 0))?;
    nth_of_class.define_method("selectors", method!(YSelectorChild::selectors, 0))?;

    let non_ts_pseudo_class = selector_class.define_class("NonTsPseudoClass", selector_child_class)?;
    non_ts_pseudo_class.define_method("kind", method!(YSelectorChild::kind, 0))?;
    non_ts_pseudo_class.define_method("type", method!(YSelectorChild::ty, 0))?;
    non_ts_pseudo_class.define_method("value", method!(YSelectorChild::value, 0))?;

    let slotted_class = selector_class.define_class("Slotted", selector_child_class)?;
    slotted_class.define_method("kind", method!(YSelectorChild::kind, 0))?;
    slotted_class.define_method("selector", method!(YSelectorChild::selector, 0))?;

    let where_class = selector_class.define_class("Where", selector_child_class)?;
    where_class.define_method("kind", method!(YSelectorChild::kind, 0))?;
    where_class.define_method("selectors", method!(YSelectorChild::selectors, 0))?;

    let is_class = selector_class.define_class("Is", selector_child_class)?;
    is_class.define_method("kind", method!(YSelectorChild::kind, 0))?;
    is_class.define_method("selectors", method!(YSelectorChild::selectors, 0))?;

    let part_class = selector_class.define_class("Part", selector_child_class)?;
    part_class.define_method("kind", method!(YSelectorChild::kind, 0))?;
    part_class.define_method("items", method!(YSelectorChild::items, 0))?;

    let has_class = selector_class.define_class("Has", selector_child_class)?;
    has_class.define_method("kind", method!(YSelectorChild::kind, 0))?;
    has_class.define_method("relative_selectors", method!(YSelectorChild::relative_selectors, 0))?;

    let host_class = selector_class.define_class("Host", selector_child_class)?;
    host_class.define_method("kind", method!(YSelectorChild::kind, 0))?;
    host_class.define_method("selector", method!(YSelectorChild::selector, 0))?;

    let invalid_class = selector_class.define_class("Invalid", selector_child_class)?;
    invalid_class.define_method("kind", method!(YSelectorChild::kind, 0))?;

    let pseudo_element_class = selector_class.define_class("PseudoElement", selector_child_class)?;
    pseudo_element_class.define_method("kind", method!(YSelectorChild::kind, 0))?;

    let relative_selector_anchor_class = selector_class.define_class("RelativeSelectorAnchor", selector_child_class)?;
    relative_selector_anchor_class.define_method("kind", method!(YSelectorChild::kind, 0))?;

    declarations::init::init(ruby, &yass_module)?;

    Ok(())
}
