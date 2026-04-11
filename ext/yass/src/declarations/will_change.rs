use magnus::{Error, RArray, Ruby, typed_data};
use style::values::computed::WillChange;
use style::values::specified::WillChangeBits;
use style_traits::ToCss;

#[magnus::wrap(class = "Yass::Declarations::WillChange")]
pub struct YWillChange {
    will_change: WillChange,
}

impl YWillChange {
    pub fn new(will_change: WillChange) -> Self {
        Self { will_change }
    }

    pub fn is_auto(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.will_change.to_css_string() == "auto"
    }

    pub fn values(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        if Self::is_auto(ruby, rb_self) {
            return Ok(ruby.ary_new());
        }

        let css = rb_self.will_change.to_css_string();
        let values = css
            .split(',')
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .collect::<Vec<_>>();
        let result = ruby.ary_new_capa(values.len());

        for value in values {
            result.push(ruby.str_new(value))?;
        }

        Ok(result)
    }

    pub fn is_stacking_context_unconditional(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.will_change.bits.contains(WillChangeBits::STACKING_CONTEXT_UNCONDITIONAL)
    }

    pub fn is_transform(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.will_change.bits.contains(WillChangeBits::TRANSFORM)
    }

    pub fn is_scroll(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.will_change.bits.contains(WillChangeBits::SCROLL)
    }

    pub fn is_contain(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.will_change.bits.contains(WillChangeBits::CONTAIN)
    }

    pub fn is_opacity(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.will_change.bits.contains(WillChangeBits::OPACITY)
    }

    pub fn is_perspective(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.will_change.bits.contains(WillChangeBits::PERSPECTIVE)
    }

    pub fn is_z_index(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.will_change.bits.contains(WillChangeBits::Z_INDEX)
    }

    pub fn is_fixpos_cb_non_svg(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.will_change.bits.contains(WillChangeBits::FIXPOS_CB_NON_SVG)
    }

    pub fn is_position(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.will_change.bits.contains(WillChangeBits::POSITION)
    }

    pub fn is_view_transition_name(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.will_change.bits.contains(WillChangeBits::VIEW_TRANSITION_NAME)
    }

    pub fn is_backdrop_root(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.will_change.bits.contains(WillChangeBits::BACKDROP_ROOT)
    }
}
