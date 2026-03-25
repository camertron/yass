use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc, typed_data, value::Id};
use style::values::specified::{CalcLengthPercentage, calc::CalcNode};
use style_traits::values::specified::AllowedNumericType;

use crate::{cached_value::CachedValue, declarations::calc::{abs::YAbs, anchor_function::YAnchorFunction, anchor_size_function::YAnchorSizeFunction, clamp::YClamp, hypot::YHypot, invert::YInvert, leaf::make_leaf, min_max::YMinMax, mod_rem::YModRem, negate::YNegate, product::YProduct, round::YRound, sign::YSign, sum::YSum}};

pub mod abs;
pub mod anchor_function;
pub mod anchor_size_function;
pub mod clamp;
pub mod hypot;
pub mod init;
pub mod invert;
pub mod leaf;
pub mod min_max;
pub mod mod_rem;
pub mod negate;
pub mod product;
pub mod round;
pub mod sign;
pub mod sum;

pub use init::*;

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Calc", mark)]
pub struct YCalc {
    clamping_mode: AllowedNumericType,
    root: CachedValue<CalcNode>
}

impl YCalc {
    pub fn new(calc_length_percentage: Box<CalcLengthPercentage>) -> Self {
        Self {
            clamping_mode: calc_length_percentage.clamping_mode,
            root: CachedValue::new(calc_length_percentage.node, |calc_node, ruby| {
                YCalc::make_node(calc_node, ruby)
            })
        }
    }

    pub fn make_node(calc_node: &CalcNode, ruby: &Ruby) -> Value {
        match calc_node {
            CalcNode::Leaf(leaf) => {
                make_leaf(leaf.clone(), ruby)
            },

            CalcNode::Negate(generic_calc_node) => {
                YNegate::new((**generic_calc_node).clone()).into_value_with(ruby)
            },

            CalcNode::Invert(generic_calc_node) => {
                YInvert::new((**generic_calc_node).clone()).into_value_with(ruby)
            },

            CalcNode::Sum(owned_slice) => {
                YSum::new(owned_slice).into_value_with(ruby)
            },

            CalcNode::Product(owned_slice) => {
                YProduct::new(owned_slice).into_value_with(ruby)
            },

            CalcNode::MinMax(owned_slice, min_max_op) => {
                YMinMax::new(owned_slice, min_max_op.clone()).into_value_with(ruby)
            },

            CalcNode::Clamp { min, center, max } => {
                YClamp::new((**min).clone(), (**center).clone(), (**max).clone()).into_value_with(ruby)
            },

            CalcNode::Round { strategy, value, step } => {
                YRound::new(strategy.clone(), (**value).clone(), (**step).clone()).into_value_with(ruby)
            },

            CalcNode::ModRem { dividend, divisor, op } => {
                YModRem::new((**dividend).clone(), (**divisor).clone(), op.clone()).into_value_with(ruby)
            },

            CalcNode::Hypot(owned_slice) => {
                YHypot::new(owned_slice).into_value_with(ruby)
            },

            CalcNode::Abs(generic_calc_node) => {
                YAbs::new((**generic_calc_node).clone()).into_value_with(ruby)
            },

            CalcNode::Sign(generic_calc_node) => {
                YSign::new((**generic_calc_node).clone()).into_value_with(ruby)
            },

            CalcNode::Anchor(generic_anchor_function) => {
                YAnchorFunction::new((**generic_anchor_function).clone()).into_value_with(ruby)
            },

            CalcNode::AnchorSize(generic_anchor_size_function) => {
                YAnchorSizeFunction::new((**generic_anchor_size_function).clone()).into_value_with(ruby)
            },
        }
    }

    pub fn clamping_mode(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.clamping_mode {
            AllowedNumericType::All => ruby.intern("all"),
            AllowedNumericType::NonNegative => ruby.intern("non_negative"),
            AllowedNumericType::AtLeastOne => ruby.intern("at_least_one"),
            AllowedNumericType::ZeroToOne => ruby.intern("zero_to_one"),
        }
    }

    pub fn root(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.root.get(ruby)
    }
}

impl DataTypeFunctions for YCalc {
    fn mark(&self, marker: &gc::Marker) {
        self.root.mark(marker);
    }
}
