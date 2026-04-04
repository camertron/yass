use magnus::{DataTypeFunctions, Error, IntoValue, RArray, Ruby, TypedData, Value, gc, typed_data, value::Id};
use style::values::specified::{Cursor, ui::{CursorImage, CursorKind}};

use crate::{cached_value::CachedValue, cached_value_list::CachedValueList, declarations::{images::image_to_value, number::YNumber}};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Cursor", mark)]
pub struct YCursor {
    images: CachedValueList<CursorImage>,
    keyword: CursorKind,
}

impl YCursor {
    pub fn new(cursor: Cursor) -> Self {
        Self {
            images: CachedValueList::new(cursor.images.to_vec(), |image, _ctx, ruby| {
                YCursorImage::new(image.clone()).into_value_with(ruby)
            }),
            keyword: cursor.keyword,
        }
    }

    pub fn images(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        rb_self.images.to_a(ruby)
    }

    pub fn keyword(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        cursor_kind_to_id(rb_self.keyword, ruby)
    }
}

impl DataTypeFunctions for YCursor {
    fn mark(&self, marker: &gc::Marker) {
        self.images.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Cursor::Image", mark)]
pub struct YCursorImage {
    image: CachedValue<style::values::specified::image::Image>,
    has_hotspot: bool,
    hotspot_x: CachedValue<style::values::specified::Number>,
    hotspot_y: CachedValue<style::values::specified::Number>,
}

impl YCursorImage {
    pub fn new(cursor_image: CursorImage) -> Self {
        Self {
            image: CachedValue::new(cursor_image.image, |image, ruby| image_to_value(image, ruby)),
            has_hotspot: cursor_image.has_hotspot,
            hotspot_x: CachedValue::new(cursor_image.hotspot_x, |hotspot_x, ruby| {
                YNumber::new(hotspot_x.get()).into_value_with(ruby)
            }),
            hotspot_y: CachedValue::new(cursor_image.hotspot_y, |hotspot_y, ruby| {
                YNumber::new(hotspot_y.get()).into_value_with(ruby)
            }),
        }
    }

    pub fn image(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.image.get(ruby)
    }

    pub fn has_hotspot(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.has_hotspot
    }

    pub fn hotspot_x(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.hotspot_x.get(ruby)
    }

    pub fn hotspot_y(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.hotspot_y.get(ruby)
    }
}

impl DataTypeFunctions for YCursorImage {
    fn mark(&self, marker: &gc::Marker) {
        self.image.mark(marker);
        self.hotspot_x.mark(marker);
        self.hotspot_y.mark(marker);
    }
}

fn cursor_kind_to_id(cursor_kind: CursorKind, ruby: &Ruby) -> Id {
    match cursor_kind {
        CursorKind::None => ruby.intern("none"),
        CursorKind::Default => ruby.intern("default"),
        CursorKind::Pointer => ruby.intern("pointer"),
        CursorKind::ContextMenu => ruby.intern("context_menu"),
        CursorKind::Help => ruby.intern("help"),
        CursorKind::Progress => ruby.intern("progress"),
        CursorKind::Wait => ruby.intern("wait"),
        CursorKind::Cell => ruby.intern("cell"),
        CursorKind::Crosshair => ruby.intern("crosshair"),
        CursorKind::Text => ruby.intern("text"),
        CursorKind::VerticalText => ruby.intern("vertical_text"),
        CursorKind::Alias => ruby.intern("alias"),
        CursorKind::Copy => ruby.intern("copy"),
        CursorKind::Move => ruby.intern("move"),
        CursorKind::NoDrop => ruby.intern("no_drop"),
        CursorKind::NotAllowed => ruby.intern("not_allowed"),
        CursorKind::Grab => ruby.intern("grab"),
        CursorKind::Grabbing => ruby.intern("grabbing"),
        CursorKind::EResize => ruby.intern("e_resize"),
        CursorKind::NResize => ruby.intern("n_resize"),
        CursorKind::NeResize => ruby.intern("ne_resize"),
        CursorKind::NwResize => ruby.intern("nw_resize"),
        CursorKind::SResize => ruby.intern("s_resize"),
        CursorKind::SeResize => ruby.intern("se_resize"),
        CursorKind::SwResize => ruby.intern("sw_resize"),
        CursorKind::WResize => ruby.intern("w_resize"),
        CursorKind::EwResize => ruby.intern("ew_resize"),
        CursorKind::NsResize => ruby.intern("ns_resize"),
        CursorKind::NeswResize => ruby.intern("nesw_resize"),
        CursorKind::NwseResize => ruby.intern("nwse_resize"),
        CursorKind::ColResize => ruby.intern("col_resize"),
        CursorKind::RowResize => ruby.intern("row_resize"),
        CursorKind::AllScroll => ruby.intern("all_scroll"),
        CursorKind::ZoomIn => ruby.intern("zoom_in"),
        CursorKind::ZoomOut => ruby.intern("zoom_out"),
        CursorKind::Auto => ruby.intern("auto"),
    }
}
