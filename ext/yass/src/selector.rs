use std::cell::RefCell;

use magnus::{DataTypeFunctions, Error, Integer, Module, RArray, RModule, RString, Ruby, TypedData, Value, gc, typed_data, value::{Id, Opaque, Qtrue, ReprValue}};
use selectors::{SelectorList, attr::{AttrSelectorOperator, AttrSelectorWithOptionalNamespace, NamespaceConstraint, ParsedCaseSensitivity}, parser::{Combinator, Component, LocalName, NthSelectorData, NthType, Selector}};
use style::{selector_parser::SelectorImpl};

#[magnus::wrap(class = "Yass::SelectorChild")]
pub enum YSelectorChild {
    #[magnus(class = "Yass::Selector::LocalName")]
    LocalName(YLocalName),

    #[magnus(class = "Yass::Selector::ID")]
    ID(YID),

    #[magnus(class = "Yass::Selector::Klass")]
    Class(YClass),

    #[magnus(class = "Yass::Selector::AttributeInNoNamespaceExists")]
    AttributeInNoNamespaceExists(YAttributeInNoNamespaceExists),

    #[magnus(class = "Yass::Selector::AttributeInNoNamespace")]
    AttributeInNoNamespace(YAttributeInNoNamespace),

    #[magnus(class = "Yass::Selector::AttributeOther")]
    AttributeOther(YAttributeOther),

    #[magnus(class = "Yass::Selector::ExplicitUniversalType")]
    ExplicitUniversalType(YExplicitUniversalType),

    #[magnus(class = "Yass::Selector::ExplicitAnyNamespace")]
    ExplicitAnyNamespace(YExplicitAnyNamespace),

    #[magnus(class = "Yass::Selector::ExplicitNoNamespace")]
    ExplicitNoNamespace(YExplicitNoNamespace),

    #[magnus(class = "Yass::Selector::DefaultNamespace")]
    DefaultNamespace(YDefaultNamespace),

    #[magnus(class = "Yass::Selector::Namespace")]
    Namespace(YNamespace),

    #[magnus(class = "Yass::Selector::Negation")]
    Negation(YNegation),

    #[magnus(class = "Yass::Selector::Root")]
    Root(YRoot),

    #[magnus(class = "Yass::Selector::Empty")]
    Empty(YEmpty),

    #[magnus(class = "Yass::Selector::Scope")]
    Scope(YScope),

    #[magnus(class = "Yass::Selector::ImplicitScope")]
    ImplicitScope(YImplicitScope),

    #[magnus(class = "Yass::Selector::ParentSelector")]
    ParentSelector(YParentSelector),

    #[magnus(class = "Yass::Selector::Nth")]
    Nth(YNth),

    #[magnus(class = "Yass::Selector::Combinator")]
    Combinator(YCombinator)
}

impl YSelectorChild {
    pub fn kind(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<Value, Error> {
        match &*rb_self {
            YSelectorChild::Combinator(ycombinator) => {
                Ok(ruby.into_value(ycombinator.kind(ruby)))
            },

            YSelectorChild::LocalName(_) => Ok(ruby.into_value(ruby.intern("local_name"))),
            YSelectorChild::ID(_) => Ok(ruby.into_value(ruby.intern("id"))),
            YSelectorChild::Class(_) => Ok(ruby.into_value(ruby.intern("class"))),
            YSelectorChild::AttributeInNoNamespaceExists(_) => Ok(ruby.into_value(ruby.intern("attribute_in_no_namespace_exists"))),
            YSelectorChild::AttributeInNoNamespace(_) => Ok(ruby.into_value(ruby.intern("attribute_in_no_namespace"))),
            YSelectorChild::AttributeOther(_) => Ok(ruby.into_value(ruby.intern("attribute_other"))),
            YSelectorChild::ExplicitUniversalType(_) => Ok(ruby.into_value(ruby.intern("explicit_universal_type"))),
            YSelectorChild::ExplicitAnyNamespace(_) => Ok(ruby.into_value(ruby.intern("explicit_any_namespace"))),
            YSelectorChild::ExplicitNoNamespace(_) => Ok(ruby.into_value(ruby.intern("explicit_no_namespace"))),
            YSelectorChild::DefaultNamespace(_) => Ok(ruby.into_value(ruby.intern("default_namespace"))),
            YSelectorChild::Namespace(_) => Ok(ruby.into_value(ruby.intern("namespace"))),
            YSelectorChild::Negation(_) => Ok(ruby.into_value(ruby.intern("negation"))),
            YSelectorChild::Root(_) => Ok(ruby.into_value(ruby.intern("root"))),
            YSelectorChild::Empty(_) => Ok(ruby.into_value(ruby.intern("empty"))),
            YSelectorChild::Scope(_) => Ok(ruby.into_value(ruby.intern("scope"))),
            YSelectorChild::ImplicitScope(_) => Ok(ruby.into_value(ruby.intern("implicit_scope"))),
            YSelectorChild::ParentSelector(_) => Ok(ruby.into_value(ruby.intern("parent_selector"))),
            YSelectorChild::Nth(_) => Ok(ruby.into_value(ruby.intern("nth"))),

            _ => Err(
                Error::new(ruby.exception_no_method_error(), "unreachable")
            )
        }
    }

    pub fn is_ancestor(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<Value, Error> {
        match &*rb_self {
            YSelectorChild::Combinator(ycombinator) => {
                Ok(ycombinator.is_ancestor(ruby))
            },

            _ => Err(
                Error::new(ruby.exception_no_method_error(), "unreachable")
            )
         }
    }

    pub fn is_pseudo_element(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<Value, Error> {
        match &*rb_self {
            YSelectorChild::Combinator(ycombinator) => {
                Ok(ycombinator.is_pseudo_element(ruby))
            },

            _ => Err(
                Error::new(ruby.exception_no_method_error(), "unreachable")
            )
         }
    }

    pub fn is_sibling(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<Value, Error> {
        match &*rb_self {
            YSelectorChild::Combinator(ycombinator) => {
                Ok(ycombinator.is_sibling(ruby))
            },

            _ => Err(
                Error::new(ruby.exception_no_method_error(), "unreachable")
            )
         }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<Value, Error> {
        match &*rb_self {
            YSelectorChild::ID(yid) => Ok(yid.value(ruby).as_value()),
            YSelectorChild::Class(yclass) => Ok(yclass.value(ruby).as_value()),
            YSelectorChild::LocalName(ylocalname) => {
                Ok(ylocalname.value(ruby).as_value())
            },

            YSelectorChild::AttributeInNoNamespaceExists(yattr) => {
                Ok(yattr.value(ruby).as_value())
            }

            YSelectorChild::AttributeInNoNamespace(yainn) => {
                Ok(yainn.value(ruby).as_value())
            }

            _ => Err(
                Error::new(ruby.exception_no_method_error(), "unreachable")
            )
        }
    }

    pub fn selectors(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<Value, Error> {
        match &*rb_self {
            YSelectorChild::Negation(ynegation) => {
                Ok(ynegation.selectors(ruby)?.as_value())
            },

            _ => Err(
                Error::new(ruby.exception_no_method_error(), "unreachable")
            )
        }
    }

    pub fn name(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<Value, Error> {
        match &*rb_self {
            YSelectorChild::AttributeInNoNamespace(yainn) => {
                Ok(yainn.name(ruby).as_value())
            }

            _ => Err(
                Error::new(ruby.exception_no_method_error(), "unreachable")
            )
        }
    }

    pub fn operator(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<Value, Error> {
        match &*rb_self {
            YSelectorChild::AttributeInNoNamespace(yainn) => {
                Ok(ruby.into_value(yainn.operator(ruby)))
            }

            _ => Err(
                Error::new(ruby.exception_no_method_error(), "unreachable")
            )
        }
    }

    pub fn case_sensitivity(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<Value, Error> {
        match &*rb_self {
            YSelectorChild::AttributeInNoNamespace(yainn) => {
                Ok(ruby.into_value(yainn.case_sensitivity(ruby)))
            }

            _ => Err(
                Error::new(ruby.exception_no_method_error(), "unreachable")
            )
        }
    }

    pub fn namespace(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<Option<Value>, Error> {
        match &*rb_self {
            YSelectorChild::AttributeOther(yattrother) => {
                yattrother.namespace(ruby)
            }

            _ => Err(
                Error::new(ruby.exception_no_method_error(), "unreachable")
            )
        }
    }

    pub fn prefix(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<Value, Error> {
        match &*rb_self {
            YSelectorChild::Namespace(ynamespace) => {
                Ok(ynamespace.prefix(ruby).as_value())
            }

            _ => Err(
                Error::new(ruby.exception_no_method_error(), "unreachable")
            )
        }
    }

    pub fn url(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<Value, Error> {
        match &*rb_self {
            YSelectorChild::Namespace(ynamespace) => {
                Ok(ynamespace.url(ruby).as_value())
            }

            _ => Err(
                Error::new(ruby.exception_no_method_error(), "unreachable")
            )
        }
    }

    pub fn ty(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<Value, Error> {
        match &*rb_self {
            YSelectorChild::Nth(ynth) => {
                Ok(ruby.into_value(ynth.ty(ruby)))
            }

            _ => Err(
                Error::new(ruby.exception_no_method_error(), "unreachable")
            )
        }
    }

    pub fn is_method(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<Value, Error> {
        match &*rb_self {
            YSelectorChild::Nth(ynth) => {
                Ok(ynth.is_function(ruby))
            }

            _ => Err(
                Error::new(ruby.exception_no_method_error(), "unreachable")
            )
        }
    }

    pub fn an_plus_b(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<Value, Error> {
        match &*rb_self {
            YSelectorChild::Nth(ynth) => {
                Ok(ynth.an_plus_b(ruby))
            }

            _ => Err(
                Error::new(ruby.exception_no_method_error(), "unreachable")
            )
        }
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Selector", mark)]
pub struct YSelector {
    selector: Selector<SelectorImpl>,
    cached_children: RefCell<Option<Vec<Opaque<typed_data::Obj<YSelectorChild>>>>>
}

impl DataTypeFunctions for YSelector {
    fn mark(&self, marker: &gc::Marker) {
        println!("Marking children");
        if let Some(children) = self.cached_children.borrow().as_ref() {
            marker.mark_slice(children.as_slice());
        }
    }
}

impl YSelector {
    pub fn new(selector: Selector<SelectorImpl>) -> Self {
        Self { selector, cached_children: RefCell::new(None) }
    }

    pub fn children(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        if rb_self.cached_children.borrow().is_none() {
            let mut iter = rb_self.selector.iter();
            let mut new_children: Vec<Opaque<typed_data::Obj<YSelectorChild>>> = vec![];

            loop {
                for raw_component in iter.by_ref() {
                    let component = match raw_component {
                        Component::LocalName(local_name) => {
                            YSelectorChild::LocalName(YLocalName::new(local_name.clone()))
                        },
                        Component::ID(id) => {
                            YSelectorChild::ID(YID::new(id.to_string()))
                        },
                        Component::Class(class) => {
                            YSelectorChild::Class(YClass::new(class.to_string()))
                        },

                        Component::AttributeInNoNamespaceExists { local_name, local_name_lower: _ } => {
                            YSelectorChild::AttributeInNoNamespaceExists(YAttributeInNoNamespaceExists::new(local_name.to_string()))
                        },

                        Component::AttributeInNoNamespace { local_name, operator, value, case_sensitivity } => {
                            YSelectorChild::AttributeInNoNamespace(
                                YAttributeInNoNamespace::new(
                                    local_name.to_string(),
                                    *operator,
                                    value.to_string(),
                                    *case_sensitivity
                                )
                            )
                        },

                        Component::AttributeOther(attr_selector_with_optional_namespace) => {
                            YSelectorChild::AttributeOther(
                                YAttributeOther::new(attr_selector_with_optional_namespace.clone())
                            )
                        },

                        Component::ExplicitUniversalType => {
                            YSelectorChild::ExplicitUniversalType(YExplicitUniversalType::new())
                        },

                        Component::ExplicitAnyNamespace => {
                            YSelectorChild::ExplicitAnyNamespace(YExplicitAnyNamespace::new())
                        },

                        Component::ExplicitNoNamespace => {
                            YSelectorChild::ExplicitNoNamespace(YExplicitNoNamespace::new())
                        },

                        Component::DefaultNamespace(ns) => {
                            YSelectorChild::DefaultNamespace(YDefaultNamespace::new(ns.to_string()))
                        },

                        Component::Namespace(prefix, url) => {
                            YSelectorChild::Namespace(YNamespace::new(prefix.to_string(), url.to_string()))
                        },

                        Component::Negation(selector_list) => {
                            YSelectorChild::Negation(YNegation::new(selector_list.clone()))
                        },

                        Component::Root => YSelectorChild::Root(YRoot::new()),
                        Component::Empty => YSelectorChild::Empty(YEmpty::new()),
                        Component::Scope => YSelectorChild::Scope(YScope::new()),
                        Component::ImplicitScope => YSelectorChild::ImplicitScope(YImplicitScope::new()),
                        Component::ParentSelector => YSelectorChild::ParentSelector(YParentSelector::new()),

                        Component::Nth(nth_selector_data) => {
                            YSelectorChild::Nth(YNth::new(nth_selector_data.clone()))
                        },
                        Component::NthOf(nth_of_selector_data) => todo!(),

                        Component::NonTSPseudoClass(_) => todo!(),
                        Component::Slotted(selector) => todo!(),
                        Component::Part(items) => todo!(),
                        Component::Host(selector) => todo!(),
                        Component::Where(selector_list) => todo!(),
                        Component::Is(selector_list) => todo!(),
                        Component::Has(relative_selectors) => todo!(),
                        Component::Invalid(_) => todo!(),
                        Component::PseudoElement(_) => todo!(),
                        Component::Combinator(combinator) => todo!(),
                        Component::RelativeSelectorAnchor => todo!(),
                    };

                    new_children.push(ruby.obj_wrap(component).into());
                }

                if let Some(combinator) = iter.next_sequence() {
                    let ycombinator = YCombinator::new(combinator);
                    new_children.push(ruby.obj_wrap(YSelectorChild::Combinator(ycombinator)).into());
                } else {
                    break;
                }
            }

            *rb_self.cached_children.borrow_mut() = Some(new_children);
        }

        let cached_children = rb_self.cached_children.borrow();
        let result = ruby.ary_new_capa(cached_children.as_ref().unwrap().len());

        for child in cached_children.as_ref().unwrap() {
            result.push(ruby.get_inner(*child))?;
        }

        return Ok(result);
    }

    fn intern_operator(operator: &AttrSelectorOperator, ruby: &Ruby) -> Id {
        match operator {
            AttrSelectorOperator::Equal => ruby.intern("equal"),
            AttrSelectorOperator::Includes => ruby.intern("includes"),
            AttrSelectorOperator::DashMatch => ruby.intern("dash_match"),
            AttrSelectorOperator::Prefix => ruby.intern("prefix"),
            AttrSelectorOperator::Substring => ruby.intern("substring"),
            AttrSelectorOperator::Suffix => ruby.intern("suffix")
        }
    }

    fn intern_case_sensitivity(cs: &ParsedCaseSensitivity, ruby: &Ruby) -> Id {
        match cs {
            // 's' was specified.
            ParsedCaseSensitivity::ExplicitCaseSensitive => ruby.intern("explicit_case_sensitive"),
            // 'i' was specified.
            ParsedCaseSensitivity::AsciiCaseInsensitive => ruby.intern("ascii_case_insensitive"),
            // No flags were specified and HTML says this is a case-sensitive attribute.
            ParsedCaseSensitivity::CaseSensitive => ruby.intern("case_sensitive"),
            // No flags were specified and HTML says this is a case-insensitive attribute.
            ParsedCaseSensitivity::AsciiCaseInsensitiveIfInHtmlElementInHtmlDocument => {
                ruby.intern("ascii_case_insensitive_if_in_html_element_in_html_document")
            }
        }
    }
}

pub struct YLocalName {
    local_name: LocalName<SelectorImpl>
}

impl YLocalName {
    pub fn new(local_name: LocalName<SelectorImpl>) -> Self {
        Self { local_name }
    }

    pub fn value(&self, ruby: &Ruby) -> RString {
        ruby.str_new(&self.local_name.name.to_string())
    }
}

pub struct YID {
    id: String
}

impl YID {
    pub fn new(id: String) -> Self {
        Self { id }
    }

    pub fn value(&self, ruby: &Ruby) -> RString {
        ruby.str_new(self.id.as_str())
    }
}

pub struct YClass {
    pub class: String
}

impl YClass {
    pub fn new(class: String) -> Self {
        Self { class }
    }

    pub fn value(&self, ruby: &Ruby) -> RString {
        ruby.str_new(self.class.as_str())
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Selector::Negation", mark)]
pub struct YNegation {
    selector_list: SelectorList<SelectorImpl>,
    cached_selectors: RefCell<Option<Vec<Opaque<typed_data::Obj<YSelector>>>>>
}

impl DataTypeFunctions for YNegation {
    fn mark(&self, marker: &gc::Marker) {
        if let Some(selectors) = self.cached_selectors.borrow().as_ref() {
            marker.mark_slice(selectors.as_slice());
        }
    }
}

impl YNegation {
    pub fn new(selector_list: SelectorList<SelectorImpl>) -> Self {
        Self { selector_list, cached_selectors: RefCell::new(None) }
    }

    pub fn selectors(&self, ruby: &Ruby) -> Result<RArray, Error> {
        if self.cached_selectors.borrow().is_none() {
            let mut new_selectors: Vec<Opaque<typed_data::Obj<YSelector>>> = vec![];

            for selector in self.selector_list.slice() {
                let yselector = YSelector::new(selector.clone());
                new_selectors.push(ruby.obj_wrap(yselector).into());
            }

            *self.cached_selectors.borrow_mut() = Some(new_selectors);
        }

        let cached_selectors = self.cached_selectors.borrow();
        let result = ruby.ary_new_capa(cached_selectors.as_ref().unwrap().len());

        for selector in cached_selectors.as_ref().unwrap() {
            result.push(ruby.get_inner(*selector))?;
        }

        Ok(result)
    }
}

pub struct YAttributeInNoNamespaceExists {
    local_name: String
}

impl YAttributeInNoNamespaceExists {
    pub fn new(local_name: String) -> Self {
        Self { local_name }
    }

    pub fn value(&self, ruby: &Ruby) -> RString {
        ruby.str_new(&self.local_name.to_string())
    }
}

pub struct YAttributeInNoNamespace {
    local_name: String,
    operator: AttrSelectorOperator,
    value: String,
    case_sensitivity: ParsedCaseSensitivity
}

impl YAttributeInNoNamespace {
    pub fn new(local_name: String, operator: AttrSelectorOperator, value: String, case_sensitivity: ParsedCaseSensitivity) -> Self {
        Self { local_name, operator, value, case_sensitivity }
    }

    pub fn name(&self, ruby: &Ruby) -> RString {
        return ruby.str_new(&self.local_name)
    }

    pub fn operator(&self, ruby: &Ruby) -> Id {
        YSelector::intern_operator(&self.operator, ruby)
    }

    pub fn value(&self, ruby: &Ruby) -> RString {
        ruby.str_new(&self.value)
    }

    pub fn case_sensitivity(&self, ruby: &Ruby) -> Id {
        return YSelector::intern_case_sensitivity(&self.case_sensitivity, ruby)
    }
}

fn singleton_instance(class_name: &str, ruby: &Ruby) -> Result<Value, Error> {
    let class = ruby
        .class_object()
        .const_get::<_, RModule>(class_name)?;

    Ok(class.funcall_public::<&str, [Value; 0], Value>("instance", [])?)
}

pub struct YAttributeOther {
    attr_selector_with_optional_namespace: Box<AttrSelectorWithOptionalNamespace<SelectorImpl>>
}

impl YAttributeOther {
    pub fn new(attr_selector_with_optional_namespace: Box<AttrSelectorWithOptionalNamespace<SelectorImpl>>) -> Self {
        Self { attr_selector_with_optional_namespace }
    }

    pub fn namespace(&self, ruby: &Ruby) -> Result<Option<Value>, Error> {
        Ok(
            if let Some(constraint) = &self.attr_selector_with_optional_namespace.namespace {
                match constraint {
                    NamespaceConstraint::Any => {
                        Some(singleton_instance("Yass::Selector::AnyNamespace", ruby)?)
                    },

                    NamespaceConstraint::Specific(constraint) => {
                        Some(
                            ruby.into_value(
                                YSpecificNamespaceConstraint::new(
                                    constraint.0.to_string(),
                                    constraint.1.to_string()
                                )
                            )
                        )
                    }
                }
            } else {
                None
            }
        )
    }
}

#[magnus::wrap(class = "Yass::Selector::SpecificNamespaceConstraint")]
pub struct YSpecificNamespaceConstraint {
    prefix: String,
    url: String
}

impl YSpecificNamespaceConstraint {
    pub fn new(prefix: String, url: String) -> Self {
        Self { prefix, url }
    }

    pub fn prefix(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> RString {
        ruby.str_new(&rb_self.prefix)
    }

    pub fn url(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> RString {
        ruby.str_new(&rb_self.url)
    }
}

pub struct YExplicitUniversalType {}

impl YExplicitUniversalType {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct YExplicitAnyNamespace {}

impl YExplicitAnyNamespace {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct YExplicitNoNamespace {}

impl YExplicitNoNamespace {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct YDefaultNamespace {
    url: String
}

impl YDefaultNamespace {
    pub fn new(url: String) -> Self {
        Self { url }
    }

    pub fn url(&self, ruby: &Ruby) -> RString {
        ruby.str_new(&self.url)
    }
}

pub struct YNamespace {
    prefix: String,
    url: String
}

impl YNamespace {
    pub fn new(prefix: String, url: String) -> Self {
        Self { prefix, url }
    }

    pub fn prefix(&self, ruby: &Ruby) -> RString {
        ruby.str_new(&self.prefix)
    }

    pub fn url(&self, ruby: &Ruby) -> RString {
        ruby.str_new(&self.url)
    }
}

pub struct YRoot {}

impl YRoot {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct YEmpty {}

impl YEmpty {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct YScope {}

impl YScope {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct YImplicitScope {}

impl YImplicitScope {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct YParentSelector {}

impl YParentSelector {
    pub fn new() -> Self {
        Self {}
    }
}

// #[derive(TypedData)]
// #[magnus(class = "Yass::Selector::Nth", mark)]
pub struct YNth {
    nth_selector_data: NthSelectorData,
    // cached_an_plus_b: RefCell<Option<Opaque<typed_data::Obj<YAnPlusB>>>>
}

impl YNth {
    pub fn new(nth_selector_data: NthSelectorData) -> Self {
        // Self { nth_selector_data, cached_an_plus_b: RefCell::new(None) }
        Self { nth_selector_data }
    }

    pub fn ty(&self, ruby: &Ruby) -> Id {
        match self.nth_selector_data.ty {
            NthType::Child => ruby.intern("child"),
            NthType::LastChild => ruby.intern("last_child"),
            NthType::OnlyChild => ruby.intern("only_child"),
            NthType::OfType => ruby.intern("of_type"),
            NthType::LastOfType => ruby.intern("last_of_type"),
            NthType::OnlyOfType => ruby.intern("only_of_type"),
        }
    }

    pub fn is_function(&self, ruby: &Ruby) -> Value {
        if self.nth_selector_data.is_function {
            ruby.qtrue().as_value()
        } else {
            ruby.qfalse().as_value()
        }
    }

    pub fn an_plus_b(&self, ruby: &Ruby) -> Value {
        // if self.cached_an_plus_b.borrow().is_none() {
        //     let an_plus_b = YAnPlusB::new(
        //         self.nth_selector_data.an_plus_b.0,
        //         self.nth_selector_data.an_plus_b.1
        //     );

        //     *self.cached_an_plus_b.borrow_mut() = Some(Opaque::from(ruby.obj_wrap(an_plus_b)));
        // }

        // let cached_an_plus_b = self.cached_an_plus_b.borrow().unwrap();
        // ruby.get_inner(cached_an_plus_b).as_value()

        ruby.obj_wrap(
            YAnPlusB::new(
                self.nth_selector_data.an_plus_b.0,
                self.nth_selector_data.an_plus_b.1
            )
        ).as_value()
    }
}

#[magnus::wrap(class = "Yass::Selector::AnPlusB")]
pub struct YAnPlusB {
    a: i32,
    b: i32
}

impl YAnPlusB {
    pub fn new(a: i32, b: i32) -> Self {
        Self { a, b }
    }

    pub fn a(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Integer {
        ruby.integer_from_i128(rb_self.a as i128)
    }

    pub fn b(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Integer {
        ruby.integer_from_i128(rb_self.b as i128)
    }
}

pub struct YCombinator {  // lol
    pub combinator: Combinator
}

impl YCombinator {
    pub fn new(combinator: Combinator) -> Self {
        YCombinator { combinator }
    }

    pub fn is_ancestor(&self, ruby: &Ruby) -> Value {
        if self.combinator.is_ancestor() {
            ruby.qtrue().as_value()
        } else {
            ruby.qfalse().as_value()
        }
    }

    pub fn is_pseudo_element(&self, ruby: &Ruby) -> Value {
        if self.combinator.is_pseudo_element() {
            ruby.qtrue().as_value()
        } else {
            ruby.qfalse().as_value()
        }
    }

    pub fn is_sibling(&self, ruby: &Ruby) -> Value {
        if self.combinator.is_sibling() {
            ruby.qtrue().as_value()
        } else {
            ruby.qfalse().as_value()
        }
    }

    pub fn kind(&self, ruby: &Ruby) -> Id {
        match self.combinator {
            Combinator::Child => ruby.intern("child"),
            Combinator::Descendant => ruby.intern("descendant"),
            Combinator::NextSibling => ruby.intern("next_sibling"),
            Combinator::LaterSibling => ruby.intern("later_sibling"),
            Combinator::PseudoElement => ruby.intern("pseudo_element"),
            Combinator::SlotAssignment => ruby.intern("slot_assignment"),
            Combinator::Part => ruby.intern("part"),
        }
    }
}
