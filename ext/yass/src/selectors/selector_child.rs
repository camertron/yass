use magnus::{DataTypeFunctions, Error, Ruby, TypedData, Value, gc, typed_data, value::ReprValue};

use crate::selectors::{
    YAttributeInNoNamespace,
    YAttributeInNoNamespaceExists,
    YAttributeOther,
    YClass,
    YCombinator,
    YDefaultNamespace,
    YEmpty,
    YExplicitAnyNamespace,
    YExplicitNoNamespace,
    YExplicitUniversalType,
    YHas,
    YHost,
    YID,
    YImplicitScope,
    YInvalid,
    YIs,
    YLocalName,
    YNamespace,
    YNegation,
    YNonTSPseudoClass,
    YNth,
    YNthOf,
    YParentSelector,
    YPart,
    YPseudoElement,
    YRelativeSelectorAnchor,
    YRoot,
    YScope,
    YSlotted,
    YWhere
};

#[derive(TypedData)]
#[magnus(class = "Yass::SelectorChild", mark)]
pub enum YSelectorChild {
    #[magnus(class = "Yass::Selector::LocalName")]
    LocalName(YLocalName),

    #[magnus(class = "Yass::Selector::Id")]
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

    #[magnus(class = "Yass::Selector::NthOf")]
    NthOf(YNthOf),

    #[magnus(class = "Yass::Selector::NonTsPseudoClass")]
    NonTSPseudoClass(YNonTSPseudoClass),

    #[magnus(class = "Yass::Selector::Slotted")]
    Slotted(YSlotted),

    #[magnus(class = "Yass::Selector::Where")]
    Where(YWhere),

    #[magnus(class = "Yass::Selector::Is")]
    Is(YIs),

    #[magnus(class = "Yass::Selector::Host")]
    Host(YHost),

    #[magnus(class = "Yass::Selector::Part")]
    Part(YPart),

    #[magnus(class = "Yass::Selector::Has")]
    Has(YHas),

    #[magnus(class = "Yass::Selector::Invalid")]
    Invalid(YInvalid),

    #[magnus(class = "Yass::Selector::Invalid")]
    PseudoElement(YPseudoElement),

    #[magnus(class = "Yass::Selector::RelativeSelectorAnchor")]
    RelativeSelectorAnchor(YRelativeSelectorAnchor),

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
            YSelectorChild::NthOf(_) => Ok(ruby.into_value(ruby.intern("nth_of"))),
            YSelectorChild::NonTSPseudoClass(_) => Ok(ruby.into_value(ruby.intern("non_ts_pseudo_class"))),
            YSelectorChild::Slotted(_) => Ok(ruby.into_value(ruby.intern("slotted"))),
            YSelectorChild::Where(_) => Ok(ruby.into_value(ruby.intern("where"))),
            YSelectorChild::Is(_) => Ok(ruby.into_value(ruby.intern("is"))),
            YSelectorChild::Part(_) => Ok(ruby.into_value(ruby.intern("part"))),
            YSelectorChild::Host(_) => Ok(ruby.into_value(ruby.intern("host"))),
            YSelectorChild::Has(_) => Ok(ruby.into_value(ruby.intern("has"))),
            YSelectorChild::Invalid(_) => Ok(ruby.into_value(ruby.intern("invalid"))),
            YSelectorChild::PseudoElement(_) => Ok(ruby.into_value(ruby.intern("pseudo_element"))),
            YSelectorChild::RelativeSelectorAnchor(_) => Ok(ruby.into_value(ruby.intern("relative_selector_anchor")))
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

            YSelectorChild::NonTSPseudoClass(ynonts) => {
                Ok(ynonts.value(ruby))
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

            YSelectorChild::NthOf(ynthof) => {
                Ok(ynthof.selectors(ruby)?.as_value())
            }

            YSelectorChild::Where(ywhere) => {
                Ok(ywhere.selectors(ruby)?.as_value())
            }

            YSelectorChild::Is(yis) => {
                Ok(yis.selectors(ruby)?.as_value())
            }

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

            YSelectorChild::DefaultNamespace(ydefaultns) => {
                Ok(ydefaultns.url(ruby).as_value())
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

            YSelectorChild::NonTSPseudoClass(ynonts) => {
                Ok(ruby.into_value(ynonts.ty(ruby)))
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

    pub fn nth(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<Value, Error> {
        match &*rb_self {
            YSelectorChild::NthOf(ynthof) => {
                Ok(ynthof.nth(ruby))
            }

            _ => Err(
                Error::new(ruby.exception_no_method_error(), "unreachable")
            )
        }
    }

    pub fn selector(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<Option<Value>, Error> {
        match &*rb_self {
            YSelectorChild::Slotted(yslotted) => {
                Ok(Some(yslotted.selector(ruby)))
            }

            YSelectorChild::Host(yhost) => {
                Ok(yhost.selector(ruby))
            }

            _ => Err(
                Error::new(ruby.exception_no_method_error(), "unreachable")
            )
        }
    }


    pub fn items(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<Value, Error> {
        match &*rb_self {
            YSelectorChild::Part(ypart) => {
                Ok(ypart.items(ruby)?.as_value())
            }

            _ => Err(
                Error::new(ruby.exception_no_method_error(), "unreachable")
            )
        }
    }

    pub fn relative_selectors(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<Value, Error> {
        match &*rb_self {
            YSelectorChild::Has(yhas) => {
                Ok(yhas.relative_selectors(ruby)?.as_value())
            }

            _ => Err(
                Error::new(ruby.exception_no_method_error(), "unreachable")
            )
        }
    }
}

impl DataTypeFunctions for YSelectorChild {
    fn mark(&self, marker: &gc::Marker) {
        match self {
            YSelectorChild::Combinator(ycombinator) => ycombinator.mark(marker),
            YSelectorChild::LocalName(local_name) => local_name.mark(marker),
            YSelectorChild::ID(id) => id.mark(marker),
            YSelectorChild::Class(klass) => klass.mark(marker),
            YSelectorChild::AttributeInNoNamespaceExists(ainne) => ainne.mark(marker),
            YSelectorChild::AttributeInNoNamespace(ainn) => ainn.mark(marker),
            YSelectorChild::AttributeOther(other) => other.mark(marker),
            YSelectorChild::ExplicitUniversalType(_ty) => (),
            YSelectorChild::ExplicitAnyNamespace(_ns) => (),
            YSelectorChild::ExplicitNoNamespace(_ns) => (),
            YSelectorChild::DefaultNamespace(ns) => ns.mark(marker),
            YSelectorChild::Namespace(_js) => (),
            YSelectorChild::Negation(neg) => neg.mark(marker),
            YSelectorChild::Root(_root) => (),
            YSelectorChild::Empty(_empty) => (),
            YSelectorChild::Scope(_scope) => (),
            YSelectorChild::ImplicitScope(_scope) => (),
            YSelectorChild::ParentSelector(_sel) => (),
            YSelectorChild::Nth(nth) => nth.mark(marker),
            YSelectorChild::NthOf(nth_of) => nth_of.mark(marker),
            YSelectorChild::NonTSPseudoClass(klass) => klass.mark(marker),
            YSelectorChild::Slotted(slotted) => slotted.mark(marker),
            YSelectorChild::Where(wh) => wh.mark(marker),
            YSelectorChild::Is(is) => is.mark(marker),
            YSelectorChild::Part(part) => part.mark(marker),
            YSelectorChild::Host(host) => host.mark(marker),
            YSelectorChild::Has(has) => has.mark(marker),
            YSelectorChild::Invalid(_invalid) => (),
            YSelectorChild::PseudoElement(_pseudo) => (),
            YSelectorChild::RelativeSelectorAnchor(_anchor) => ()
        }
    }
}
