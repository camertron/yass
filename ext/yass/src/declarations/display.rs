use magnus::{Ruby, typed_data, value::Id};
use style::values::{computed::Display, specified::box_::{DisplayInside, DisplayOutside}};

#[magnus::wrap(class = "Yass::Declarations::Display")]
pub struct YDisplay {
    display: Display
}

impl YDisplay {
    pub fn new(display: Display) -> Self {
        Self { display }
    }

    pub fn inside(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.display.inside() {
            DisplayInside::None => ruby.intern("none"),
            DisplayInside::Contents => ruby.intern("contents"),
            DisplayInside::Flow => ruby.intern("flow"),
            DisplayInside::FlowRoot => ruby.intern("flow_root"),
            DisplayInside::Flex => ruby.intern("flex"),
            DisplayInside::Grid => ruby.intern("grid"),
            DisplayInside::Table => ruby.intern("table"),
            DisplayInside::TableRowGroup => ruby.intern("table_row_group"),
            DisplayInside::TableColumn => ruby.intern("table_column"),
            DisplayInside::TableColumnGroup => ruby.intern("table_column_group"),
            DisplayInside::TableHeaderGroup => ruby.intern("table_header_group"),
            DisplayInside::TableFooterGroup => ruby.intern("table_footer_group"),
            DisplayInside::TableRow => ruby.intern("table_row"),
            DisplayInside::TableCell => ruby.intern("table_cell"),
        }
    }

    pub fn outside(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.display.outside() {
            DisplayOutside::None => ruby.intern("none"),
            DisplayOutside::Inline => ruby.intern("inline"),
            DisplayOutside::Block => ruby.intern("block"),
            DisplayOutside::TableCaption => ruby.intern("table_caption"),
            DisplayOutside::InternalTable => ruby.intern("internal_table"),
        }
    }

    pub fn is_list_item(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.display.is_list_item()
    }

    pub fn is_contents(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.display.is_contents()
    }

    pub fn is_none(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.display.is_none()
    }

    pub fn is_inline_flow(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.display.is_inline_flow()
    }

    pub fn is_item_container(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.display.is_item_container()
    }

    pub fn is_line_participant(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.display.is_line_participant()
    }

    pub fn is_ruby_level_container(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.display.is_ruby_level_container()
    }

    pub fn is_ruby_type(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.display.is_ruby_type()
    }
}
