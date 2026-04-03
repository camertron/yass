use magnus::{Ruby, Value, value::{Id, ReprValue}};
use style::selector_parser::NonTSPseudoClass;

#[magnus::wrap(class = "Yass::Selector::NonTsPseudoClass")]
pub struct YNonTSPseudoClass {
    non_ts_pseudo_class: NonTSPseudoClass
}

impl YNonTSPseudoClass {
    pub fn new(non_ts_pseudo_class: NonTSPseudoClass) -> Self {
        Self { non_ts_pseudo_class }
    }

    pub fn ty(&self, ruby: &Ruby) -> Id {
        match self.non_ts_pseudo_class {
            NonTSPseudoClass::Active => ruby.intern("active"),
            NonTSPseudoClass::AnyLink => ruby.intern("any_link"),
            NonTSPseudoClass::Autofill => ruby.intern("autofill"),
            NonTSPseudoClass::Checked => ruby.intern("checked"),
            NonTSPseudoClass::CustomState(_) => ruby.intern("custom_state"),
            NonTSPseudoClass::Default => ruby.intern("default"),
            NonTSPseudoClass::Defined => ruby.intern("defined"),
            NonTSPseudoClass::Disabled => ruby.intern("disabled"),
            NonTSPseudoClass::Enabled => ruby.intern("enabled"),
            NonTSPseudoClass::Focus => ruby.intern("focus"),
            NonTSPseudoClass::FocusWithin => ruby.intern("focus_within"),
            NonTSPseudoClass::FocusVisible => ruby.intern("focus_visible"),
            NonTSPseudoClass::Fullscreen => ruby.intern("fullscreen"),
            NonTSPseudoClass::Hover => ruby.intern("hover"),
            NonTSPseudoClass::InRange => ruby.intern("in_range"),
            NonTSPseudoClass::Indeterminate => ruby.intern("indeterminate"),
            NonTSPseudoClass::Invalid => ruby.intern("invalid"),
            NonTSPseudoClass::Lang(_) => ruby.intern("lang"),
            NonTSPseudoClass::Link => ruby.intern("link"),
            NonTSPseudoClass::Modal => ruby.intern("modal"),
            NonTSPseudoClass::MozMeterOptimum => ruby.intern("moz_meter_optimum"),
            NonTSPseudoClass::MozMeterSubOptimum => ruby.intern("moz_meter_sub_optimum"),
            NonTSPseudoClass::MozMeterSubSubOptimum => ruby.intern("moz_meter_sub_sub_optimum"),
            NonTSPseudoClass::Open => ruby.intern("open"),
            NonTSPseudoClass::Optional => ruby.intern("optional"),
            NonTSPseudoClass::OutOfRange => ruby.intern("out_of_range"),
            NonTSPseudoClass::PlaceholderShown => ruby.intern("placeholder_shown"),
            NonTSPseudoClass::PopoverOpen => ruby.intern("popover_open"),
            NonTSPseudoClass::ReadOnly => ruby.intern("read_only"),
            NonTSPseudoClass::ReadWrite => ruby.intern("read_write"),
            NonTSPseudoClass::Required => ruby.intern("required"),
            NonTSPseudoClass::ServoNonZeroBorder => ruby.intern("servo_non_zero_border"),
            NonTSPseudoClass::Target => ruby.intern("target"),
            NonTSPseudoClass::UserInvalid => ruby.intern("user_invalid"),
            NonTSPseudoClass::UserValid => ruby.intern("user_valid"),
            NonTSPseudoClass::Valid => ruby.intern("valid"),
            NonTSPseudoClass::Visited => ruby.intern("visited"),
        }
    }

    pub fn value(&self, ruby: &Ruby) -> Value {
        match &self.non_ts_pseudo_class {
            NonTSPseudoClass::CustomState(state) => ruby.str_new(&state.0.to_string()).as_value(),
            NonTSPseudoClass::Lang(lang) => ruby.str_new(&lang.to_string()).as_value(),
            _ => ruby.qnil().as_value()
        }
    }
}
