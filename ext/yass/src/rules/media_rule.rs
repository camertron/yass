use cssparser::SourceLocation;
use magnus::{DataTypeFunctions, Error, IntoValue, RArray, RString, Ruby, TypedData, Value, gc, typed_data, value::Id};
use style::{media_queries::{MediaQuery, MediaQueryType, MediaType, Qualifier}, queries::{QueryCondition, QueryFeatureExpression, condition::{Operator, StyleFeature, StyleQuery}}, servo_arc::Arc, shared_lock::SharedRwLock, stylesheets::{CssRule, MediaRule}, values::DashedIdent};
use style_traits::ToCss;

use crate::{cached_value::CachedValue, cached_value_list::CachedValueList, general::YSourceLocation, rules::rule::make_rule};

fn query_condition_to_value(condition: &QueryCondition, ruby: &Ruby) -> Value {
    match condition {
        QueryCondition::Feature(query_feature_expression) => {
            YQueryConditionFeatureExpression::new(query_feature_expression.clone()).into_value_with(ruby)
        },

        QueryCondition::Custom(dashed_ident) => {
            YQueryConditionCustom::new(dashed_ident.clone()).into_value_with(ruby)
        },

        QueryCondition::Not(query_condition) => {
            query_condition_to_value(&*query_condition, ruby)
        },

        QueryCondition::Operation(query_conditions, operator) => {
            YQueryConditionOperation::new(operator.clone(), query_conditions.clone()).into_value_with(ruby)
        },

        QueryCondition::InParens(query_condition) => {
            YQueryConditionInParens::new(*query_condition.clone()).into_value_with(ruby)
        },

        QueryCondition::Style(style_query) => {
            YQueryConditionStyle::new(style_query.clone()).into_value_with(ruby)
        },

        QueryCondition::MozPref(_moz_pref_feature) => unimplemented!(),

        QueryCondition::GeneralEnclosed(str, url) => {
            YQueryConditionGenerallyEnclosed::new(str.clone(), url.0.to_string()).into_value_with(ruby)
        },
    }
}

fn style_query_to_value(style_query: &StyleQuery, ruby: &Ruby) -> Value {
    match style_query {
        StyleQuery::Not(style_query) => {
            YStyleQueryNot::new(*style_query.clone()).into_value_with(ruby)
        },

        StyleQuery::Operation(items, operator) => {
            YStyleQueryOperation::new(*operator, items.clone()).into_value_with(ruby)
        },

        StyleQuery::InParens(style_query) => {
            YStyleQueryInParens::new(*style_query.clone()).into_value_with(ruby)
        },

        StyleQuery::Feature(style_feature) => {
            YStyleQueryStyleFeature::new(style_feature.clone()).into_value_with(ruby)
        },

        StyleQuery::GeneralEnclosed(str) => {
            YStyleQueryGenerallyEnclosed::new(str.clone()).into_value_with(ruby)
        }
    }
}

fn operator_to_id(operator: &Operator, ruby: &Ruby) -> Id {
    match operator {
        Operator::And => ruby.intern("and"),
        Operator::Or => ruby.intern("or")
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::StyleQuery::Not", mark)]
pub struct YStyleQueryNot {
    style_query: CachedValue<StyleQuery>
}

impl YStyleQueryNot {
    pub fn new(style_query: StyleQuery) -> Self {
        Self {
            style_query: CachedValue::new(style_query, |style_query, ruby| {
                style_query_to_value(style_query, ruby)
            })
        }
    }

    pub fn style_query(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.style_query.get(ruby)
    }
}

impl DataTypeFunctions for YStyleQueryNot {
    fn mark(&self, marker: &gc::Marker) {
        self.style_query.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::StyleQuery::Operation", mark)]
pub struct YStyleQueryOperation {
    operator: Operator,
    style_queries: CachedValueList<StyleQuery>,
}

impl YStyleQueryOperation {
    pub fn new(operator: Operator, style_queries: Box<[StyleQuery]>) -> Self {
        Self {
            operator,

            style_queries: CachedValueList::new(style_queries.to_vec(), |style_query, _ctx, ruby| {
                style_query_to_value(style_query, ruby)
            })
        }
    }

    pub fn style_query(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        rb_self.style_queries.to_a(ruby)
    }

    pub fn operator(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        operator_to_id(&rb_self.operator, ruby)
    }
}

impl DataTypeFunctions for YStyleQueryOperation {
    fn mark(&self, marker: &gc::Marker) {
        self.style_queries.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::StyleQuery::InParens", mark)]
pub struct YStyleQueryInParens {
    style_query: CachedValue<StyleQuery>
}

impl YStyleQueryInParens {
    pub fn new(style_query: StyleQuery) -> Self {
        Self {
            style_query: CachedValue::new(style_query, |style_query, ruby| {
                style_query_to_value(style_query, ruby)
            })
        }
    }

    pub fn style_query(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.style_query.get(ruby)
    }
}

impl DataTypeFunctions for YStyleQueryInParens {
    fn mark(&self, marker: &gc::Marker) {
        self.style_query.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::StyleQuery::StyleFeature")]
pub struct YStyleQueryStyleFeature {
    style_feature: StyleFeature
}

impl YStyleQueryStyleFeature {
    pub fn new(style_feature: StyleFeature) -> Self {
        Self { style_feature }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> RString {
        ruby.str_new(&rb_self.style_feature.to_css_string())
    }
}

#[magnus::wrap(class = "Yass::StyleQuery::GenerallyEnclosed")]
pub struct YStyleQueryGenerallyEnclosed {
    str: String
}

impl YStyleQueryGenerallyEnclosed {
    pub fn new(str: String) -> Self {
        Self { str }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> RString {
        ruby.str_new(&rb_self.str)
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::MediaRule", mark)]
pub struct YMediaRule {
    rule: Arc<MediaRule>,
    lock: SharedRwLock,
    media_queries: CachedValueList<MediaQuery>,
    rules: CachedValueList<CssRule, SharedRwLock>,
    source_location: CachedValue<SourceLocation>,
}

impl YMediaRule {
    pub fn new(rule: Arc<MediaRule>, lock: SharedRwLock) -> Self {
        Self {
            rule,
            lock: lock.clone(),

            media_queries: CachedValueList::empty(None, |query, _ctx, ruby| {
                YMediaQuery::new(query.clone()).into_value_with(ruby)
            }),

            rules: CachedValueList::empty(Some(lock.clone()), |rule, lock, ruby| {
                make_rule(rule, &lock.as_ref().unwrap(), ruby).into_value_with(ruby)
            }),

            source_location: CachedValue::empty(|source_location, ruby| {
                YSourceLocation::new(source_location.line, source_location.column).into_value_with(ruby)
            })
        }
    }

    pub fn media_queries(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        if rb_self.media_queries.is_empty() {
            let guard = rb_self.lock.read();
            let media_list = rb_self.rule.media_queries.read_with(&guard);

            for query in &media_list.media_queries {
                rb_self.media_queries.add(query.clone(), ruby)?;
            }
        }

        rb_self.media_queries.to_a(ruby)
    }

    pub fn rules(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        if rb_self.rules.is_empty() {
            let guard = rb_self.lock.read();
            let css_rules = rb_self.rule.rules.read_with(&guard);

            for css_rule in &css_rules.0 {
                rb_self.rules.add(css_rule.clone(), ruby)?;
            }
        }

        rb_self.rules.to_a(ruby)
    }

    pub fn source_location(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        if rb_self.source_location.is_empty() {
            rb_self.source_location.set(rb_self.rule.source_location, ruby);
        }

        rb_self.source_location.get(ruby)
    }
}

impl DataTypeFunctions for YMediaRule {
    fn mark(&self, marker: &gc::Marker) {
        self.media_queries.mark(marker);
        self.rules.mark(marker);
        self.source_location.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::MediaQuery", mark)]
pub struct YMediaQuery {
    qualifier: Option<Qualifier>,
    media_query_type: CachedValue<MediaQueryType>,
    query_condition: CachedValue<Option<QueryCondition>>
}

impl YMediaQuery {
    pub fn new(media_query: MediaQuery) -> Self {
        Self {
            qualifier: media_query.qualifier,
            media_query_type: CachedValue::new(media_query.media_type, |media_type, ruby| {
                match media_type {
                    MediaQueryType::All => YMediaTypeAll::new().into_value_with(ruby),
                    MediaQueryType::Concrete(media_type) => {
                        YMediaTypeConcrete::new(media_type.clone()).into_value_with(ruby)
                    }
                }
            }),

            query_condition: CachedValue::new(media_query.condition, |condition, ruby| {
                if let Some(cond) = condition {
                    query_condition_to_value(cond, ruby)
                } else {
                    ruby.qnil().into_value_with(ruby)
                }
            })
        }
    }

    pub fn qualifier(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Option<Id> {
        rb_self.qualifier.map(|qualifier| {
            match qualifier {
                Qualifier::Only => ruby.intern("only"),
                Qualifier::Not => ruby.intern("not"),
            }
        })
    }

    pub fn media_type(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.media_query_type.get(ruby)
    }

    pub fn query_condition(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.query_condition.get(ruby)
    }
}

impl DataTypeFunctions for YMediaQuery {
    fn mark(&self, marker: &gc::Marker) {
        self.media_query_type.mark(marker);
        self.query_condition.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::MediaType::All")]
pub struct YMediaTypeAll {}

impl YMediaTypeAll {
    pub fn new() -> Self {
        YMediaTypeAll {}
    }
}

#[magnus::wrap(class = "Yass::MediaType::Concrete")]
pub struct YMediaTypeConcrete {
    media_type: MediaType
}

impl YMediaTypeConcrete {
    pub fn new(media_type: MediaType) -> Self {
        Self { media_type }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> RString {
        ruby.str_new(&rb_self.media_type.0.to_css_string())
    }
}

#[magnus::wrap(class = "Yass::MediaQuery::QueryCondition::FeatureExpression")]
pub struct YQueryConditionFeatureExpression {
    query_feature_expression: QueryFeatureExpression
}

impl YQueryConditionFeatureExpression {
    pub fn new(query_feature_expression: QueryFeatureExpression) -> Self {
        Self { query_feature_expression }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> RString {
        ruby.str_new(&rb_self.query_feature_expression.to_css_string())
    }
}

#[magnus::wrap(class = "Yass::MediaQuery::QueryCondition::Custom")]
pub struct YQueryConditionCustom {
    ident: DashedIdent
}

impl YQueryConditionCustom {
    pub fn new(ident: DashedIdent) -> Self {
        Self { ident }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> RString {
        ruby.str_from_slice(&rb_self.ident.0.as_bytes())
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::MediaQuery::QueryCondition::Operation", mark)]
pub struct YQueryConditionOperation {
    operator: Operator,
    query_conditions: CachedValueList<QueryCondition>
}

impl YQueryConditionOperation {
    pub fn new(operator: Operator, query_conditions: Box<[QueryCondition]>) -> Self {
        Self {
            operator,

            query_conditions: CachedValueList::new(query_conditions.to_vec(), |query_condition, _ctx, ruby| {
                query_condition_to_value(query_condition, ruby)
            })
        }
    }

    pub fn operator(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        operator_to_id(&rb_self.operator, ruby)
    }

    pub fn query_conditions(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        rb_self.query_conditions.to_a(ruby)
    }
}

impl DataTypeFunctions for YQueryConditionOperation {
    fn mark(&self, marker: &gc::Marker) {
        self.query_conditions.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::MediaQuery::QueryCondition::InParens", mark)]
pub struct YQueryConditionInParens {
    query_condition: CachedValue<QueryCondition>
}

impl YQueryConditionInParens {
    pub fn new(query_condition: QueryCondition) -> Self {
        Self {
            query_condition: CachedValue::new(query_condition, |query_condition, ruby| {
                query_condition_to_value(query_condition, ruby)
            })
        }
    }

    pub fn query_condition(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.query_condition.get(ruby)
    }
}

impl DataTypeFunctions for YQueryConditionInParens {
    fn mark(&self, marker: &gc::Marker) {
        self.query_condition.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::MediaQuery::QueryCondition::Style", mark)]
pub struct YQueryConditionStyle {
    style_query: CachedValue<StyleQuery>
}

impl YQueryConditionStyle {
    pub fn new(style_query: StyleQuery) -> Self {
        YQueryConditionStyle {
            style_query: CachedValue::new(style_query, |style_query, ruby| {
                style_query_to_value(style_query, ruby)
            })
        }
    }

    pub fn style_query(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.style_query.get(ruby)
    }
}

impl DataTypeFunctions for YQueryConditionStyle {
    fn mark(&self, marker: &gc::Marker) {
        self.style_query.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::MediaQuery::QueryCondition::GenerallyEnclosed")]
pub struct YQueryConditionGenerallyEnclosed {
    str: String,
    url: String,
}

impl YQueryConditionGenerallyEnclosed {
    pub fn new(str: String, url: String) -> Self {
        Self { str, url: url }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> RString {
        ruby.str_new(&rb_self.str)
    }

    pub fn url(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> RString {
        ruby.str_new(&rb_self.url)
    }
}
