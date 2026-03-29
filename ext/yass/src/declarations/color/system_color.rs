use magnus::{Ruby, typed_data, value::Id};
use style::values::specified::color::SystemColor;

#[magnus::wrap(class = "Yass::Declarations::Color::SystemColor")]
pub struct YSystemColor {
    system_color: SystemColor
}

impl YSystemColor {
    pub fn new(system_color: SystemColor) -> Self {
        Self { system_color }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.system_color {
            SystemColor::Accentcolor => ruby.intern("accent_color"),
            SystemColor::Accentcolortext => ruby.intern("accent_color_text"),
            SystemColor::Activetext => ruby.intern("active_text"),
            SystemColor::Linktext => ruby.intern("link_text"),
            SystemColor::Visitedtext => ruby.intern("visited_text"),
            SystemColor::Buttonborder => ruby.intern("button_border"),
            SystemColor::Buttonface => ruby.intern("button_face"),
            SystemColor::Buttontext => ruby.intern("button_text"),
            SystemColor::Canvas => ruby.intern("canvas"),
            SystemColor::Canvastext => ruby.intern("canvas_text"),
            SystemColor::Field => ruby.intern("field"),
            SystemColor::Fieldtext => ruby.intern("field_text"),
            SystemColor::Graytext => ruby.intern("gray_text"),
            SystemColor::Highlight => ruby.intern("highlight"),
            SystemColor::Highlighttext => ruby.intern("highlight_text"),
            SystemColor::Mark => ruby.intern("mark"),
            SystemColor::Marktext => ruby.intern("mark_text"),
            SystemColor::Selecteditem => ruby.intern("selected_item"),
            SystemColor::Selecteditemtext => ruby.intern("selected_item_text"),
            SystemColor::Activeborder => ruby.intern("active_border"),
            SystemColor::Inactiveborder => ruby.intern("inactive_border"),
            SystemColor::Threeddarkshadow => ruby.intern("threed_dark_shadow"),
            SystemColor::Threedhighlight => ruby.intern("threed_highlight"),
            SystemColor::Threedlightshadow => ruby.intern("threed_light_shadow"),
            SystemColor::Threedshadow => ruby.intern("threed_shadow"),
            SystemColor::Windowframe => ruby.intern("window_frame"),
            SystemColor::Buttonhighlight => ruby.intern("button_highlight"),
            SystemColor::Buttonshadow => ruby.intern("button_shadow"),
            SystemColor::Threedface => ruby.intern("threed_face"),
            SystemColor::Activecaption => ruby.intern("active_caption"),
            SystemColor::Appworkspace => ruby.intern("app_workspace"),
            SystemColor::Background => ruby.intern("background"),
            SystemColor::Inactivecaption => ruby.intern("inactive_caption"),
            SystemColor::Infobackground => ruby.intern("info_background"),
            SystemColor::Menu => ruby.intern("menu"),
            SystemColor::Scrollbar => ruby.intern("scrollbar"),
            SystemColor::Window => ruby.intern("window"),
            SystemColor::Captiontext => ruby.intern("caption_text"),
            SystemColor::Infotext => ruby.intern("info_text"),
            SystemColor::Menutext => ruby.intern("menu_text"),
            SystemColor::Windowtext => ruby.intern("window_text"),
            SystemColor::Inactivecaptiontext => ruby.intern("inactive_caption_text"),
        }
    }
}
