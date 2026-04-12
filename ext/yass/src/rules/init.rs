use magnus::{Error, Module, RModule, Ruby, method};

use crate::rules::{media_rule::{YMediaQuery, YMediaRule, YMediaTypeConcrete, YQueryConditionCustom, YQueryConditionFeatureExpression, YQueryConditionGenerallyEnclosed, YQueryConditionInParens, YQueryConditionOperation, YQueryConditionStyle, YStyleQueryGenerallyEnclosed, YStyleQueryInParens, YStyleQueryNot, YStyleQueryOperation, YStyleQueryStyleFeature}, style_rule::YStyleRule};

pub fn init(ruby: &Ruby, yass_module: &RModule) -> Result<(), Error> {
    let rule_class = yass_module.define_class("Rule", ruby.class_object())?;

    let style_rule_class = yass_module.define_class("StyleRule", rule_class)?;
    style_rule_class.define_method("selectors", method!(YStyleRule::selectors, 0))?;
    style_rule_class.define_method("declarations", method!(YStyleRule::declarations, 0))?;

    let media_rule_class = yass_module.define_class("MediaRule", rule_class)?;
    media_rule_class.define_method("media_queries", method!(YMediaRule::media_queries, 0))?;
    media_rule_class.define_method("rules", method!(YMediaRule::rules, 0))?;

    let media_query_class = yass_module.define_class("MediaQuery", ruby.class_object())?;
    media_query_class.define_method("qualifier", method!(YMediaQuery::qualifier, 0))?;
    media_query_class.define_method("media_type", method!(YMediaQuery::media_type, 0))?;
    media_query_class.define_method("query_condition", method!(YMediaQuery::query_condition, 0))?;

    let media_type_module = yass_module.define_module("MediaType")?;
    let _media_type_all_class = media_type_module.define_class("All", ruby.class_object())?;

    let media_type_concrete_class = media_type_module.define_class("Concrete", ruby.class_object())?;
    media_type_concrete_class.define_method("value", method!(YMediaTypeConcrete::value, 0))?;

    let media_query_condition_module = media_query_class.define_module("QueryCondition")?;

    let media_query_condition_feature_expression_class = media_query_condition_module.define_class("FeatureExpression", ruby.class_object())?;
    media_query_condition_feature_expression_class.define_method("value", method!(YQueryConditionFeatureExpression::value, 0))?;

    let media_query_condition_custom_class = media_query_condition_module.define_class("Custom", ruby.class_object())?;
    media_query_condition_custom_class.define_method("value", method!(YQueryConditionCustom::value, 0))?;

    let media_query_condition_operation_class = media_query_condition_module.define_class("Operation", ruby.class_object())?;
    media_query_condition_operation_class.define_method("operator", method!(YQueryConditionOperation::operator, 0))?;
    media_query_condition_operation_class.define_method("query_conditions", method!(YQueryConditionOperation::query_conditions, 0))?;

    let media_query_condition_in_parens_class = media_query_condition_module.define_class("InParens", ruby.class_object())?;
    media_query_condition_in_parens_class.define_method("query_condition", method!(YQueryConditionInParens::query_condition, 0))?;

    let media_query_condition_style_class = media_query_condition_module.define_class("Style", ruby.class_object())?;
    media_query_condition_style_class.define_method("style_query", method!(YQueryConditionStyle::style_query, 0))?;

    let media_query_condition_generally_enclosed_class = media_query_condition_module.define_class("GenerallyEnclosed", ruby.class_object())?;
    media_query_condition_generally_enclosed_class.define_method("value", method!(YQueryConditionGenerallyEnclosed::value, 0))?;

    let style_query_module = yass_module.define_class("StyleQuery", ruby.class_object())?;

    let style_query_not_class = style_query_module.define_class("Not", ruby.class_object())?;
    style_query_not_class.define_method("style_query", method!(YStyleQueryNot::style_query, 0))?;

    let style_query_operation_class = style_query_module.define_class("Operation", ruby.class_object())?;
    style_query_operation_class.define_method("style_query", method!(YStyleQueryOperation::style_query, 0))?;
    style_query_operation_class.define_method("operator", method!(YStyleQueryOperation::operator, 0))?;

    let style_query_in_parens_class = style_query_module.define_class("InParens", ruby.class_object())?;
    style_query_in_parens_class.define_method("style_query", method!(YStyleQueryInParens::style_query, 0))?;

    let style_query_style_feature_class = style_query_module.define_class("StyleFeature", ruby.class_object())?;
    style_query_style_feature_class.define_method("value", method!(YStyleQueryStyleFeature::value, 0))?;

    let style_query_generally_enclosed_class = style_query_module.define_class("GenerallyEnclosed", ruby.class_object())?;
    style_query_generally_enclosed_class.define_method("value", method!(YStyleQueryGenerallyEnclosed::value, 0))?;

    let _unimplemented_rule_class = yass_module.define_class("UnimplementedRule", rule_class)?;

    Ok(())
}