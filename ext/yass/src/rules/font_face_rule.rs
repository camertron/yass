use cssparser::UnicodeRange;
use magnus::{DataTypeFunctions, Error, IntoValue, RArray, Ruby, TypedData, Value, gc, typed_data, value::Id};
use style::{font_face::{FontDisplay, FontStretchRange, FontStyle, FontWeightRange, Source, SourceList}, servo_arc::Arc, shared_lock::{Locked, SharedRwLock}, stylesheets::FontFaceRule, values::{computed::font::FamilyName, generics::NonNegative, specified::{Percentage, font::MetricsOverride}}};

use crate::{cached_value::CachedValue, cached_value_list::CachedValueList, declarations::percentage::YPercentage, general::YUnicodeRange, optional_cached_value::OptionalCachedValue, rules::fonts::{family::YFontFamilyName, metrics::{YFontMetricsOverride, YFontMetricsOverrideNormal}, source::font_source_to_value, stretch::YFontStretchRange, style::font_style_to_value, weight::YFontWeightRange}};

fn metrics_override_to_value(metrics_override: &MetricsOverride, ruby: &Ruby) -> Value {
    match *metrics_override {
        MetricsOverride::Override(percentage) => {
            YFontMetricsOverride::new(percentage.0).into_value_with(ruby)
        },

        MetricsOverride::Normal => {
            YFontMetricsOverrideNormal::new().into_value_with(ruby)
        },
    }
}

fn font_display_to_id(font_display: &FontDisplay, ruby: &Ruby) -> Id {
    match font_display {
        FontDisplay::Auto => ruby.intern("auto"),
        FontDisplay::Block => ruby.intern("block"),
        FontDisplay::Swap => ruby.intern("swap"),
        FontDisplay::Fallback => ruby.intern("fallback"),
        FontDisplay::Optional => ruby.intern("optional"),
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::FontFaceRule", mark)]
pub struct YFontFaceRule {
    ascent_override: OptionalCachedValue<MetricsOverride>,
    descent_override: OptionalCachedValue<MetricsOverride>,
    display: OptionalCachedValue<FontDisplay>,
    family: OptionalCachedValue<FamilyName>,
    font_face: Option<CachedValue<(FamilyName, SourceList)>>,
    language_override: Option<u32>,
    line_gap_override: OptionalCachedValue<MetricsOverride>,
    size_adjust: OptionalCachedValue<NonNegative<Percentage>>,
    sources: Option<CachedValueList<Source>>,
    font_stretch_range: OptionalCachedValue<FontStretchRange>,
    style: OptionalCachedValue<FontStyle>,
    unicode_range: Option<CachedValueList<UnicodeRange>>,
    weight: OptionalCachedValue<FontWeightRange>
}

impl YFontFaceRule {
    pub fn new(rule: Arc<Locked<FontFaceRule>>, lock: SharedRwLock) -> Self {
        let guard = lock.read();
        let font_face_rule = rule.read_with(&guard);

        Self {
            ascent_override: OptionalCachedValue::new(font_face_rule.ascent_override, |ascent_override, ruby| {
                metrics_override_to_value(ascent_override, ruby)
            }),

            descent_override: OptionalCachedValue::new(font_face_rule.descent_override, |descent_override, ruby| {
                metrics_override_to_value(descent_override, ruby)
            }),

            display: OptionalCachedValue::new(font_face_rule.display, |display, ruby| {
                font_display_to_id(display, ruby).into_value_with(ruby)
            }),

            // @TODO: Why do we have to clone twice here??
            family: OptionalCachedValue::new(font_face_rule.family.clone(), |family, ruby| {
                YFontFamilyName::new(family.clone()).into_value_with(ruby)
            }),

            font_face: if font_face_rule.font_face().is_some() {
                Some(
                    CachedValue::new((font_face_rule.font_face().as_ref().unwrap().family().clone(), font_face_rule.font_face().as_ref().unwrap().sources().clone()), |(family, sources), ruby| {
                        YFontFace::new(family.clone(), sources.clone()).into_value_with(ruby)
                    })
                )
            } else {
                None
            },

            language_override: font_face_rule.language_override.map(|lo| lo.0),

            line_gap_override: OptionalCachedValue::new(font_face_rule.line_gap_override, |line_gap_override, ruby| {
                metrics_override_to_value(line_gap_override, ruby)
            }),

            size_adjust: OptionalCachedValue::new(font_face_rule.size_adjust, |size_adjust, ruby| {
                YPercentage::new(size_adjust.0.get()).into_value_with(ruby)
            }),

            sources: if font_face_rule.sources.is_some() {
                Some(
                    CachedValueList::new(font_face_rule.sources.as_ref().unwrap().0.clone(), |source, _ctx, ruby| {
                        font_source_to_value(source, ruby)
                    })
                )
            } else {
                None
            },

            font_stretch_range: OptionalCachedValue::new(font_face_rule.stretch.clone(), |stretch, ruby| {
                YFontStretchRange::new(stretch.clone()).into_value_with(ruby)
            }),

            style: OptionalCachedValue::new(font_face_rule.style.clone(), |style, ruby| {
                font_style_to_value(style, ruby)
            }),

            unicode_range: if font_face_rule.unicode_range.is_some() {
                Some(
                    CachedValueList::new(font_face_rule.unicode_range.as_ref().unwrap().clone(), |range, _ctx, ruby| {
                        YUnicodeRange::new(range.start, range.end).into_value_with(ruby)
                    })
                )
            } else {
                None
            },

            weight: OptionalCachedValue::new(font_face_rule.weight.clone(), |weight, ruby| {
                YFontWeightRange::new(weight.clone()).into_value_with(ruby)
            })
        }
    }

    pub fn ascent_override(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.ascent_override.get(ruby)
    }

    pub fn descent_override(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.descent_override.get(ruby)
    }

    pub fn display(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.display.get(ruby)
    }

    pub fn family(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.family.get(ruby)
    }

    pub fn font_face(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Option<Value> {
        rb_self.font_face.as_ref().map(|font_face| font_face.get(ruby))
    }

    pub fn language_override(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Option<u32> {
        rb_self.language_override
    }

    pub fn line_gap_override(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.line_gap_override.get(ruby)
    }

    pub fn size_adjust(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.size_adjust.get(ruby)
    }

    pub fn sources(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<Option<RArray>, Error> {
        if let Some(sources) = &rb_self.sources {
            if let Ok(arr) = sources.to_a(ruby) {
                return Ok(Some(arr));
            }
        }

        Ok(None)
    }

    pub fn font_stretch_range(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.font_stretch_range.get(ruby)
    }

    pub fn style(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.style.get(ruby)
    }

    pub fn unicode_range(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<Option<RArray>, Error> {
        if let Some(unicode_range) = &rb_self.unicode_range {
            if let Ok(items) = unicode_range.to_a(ruby) {
                return Ok(Some(items));
            }
        }

        Ok(None)
    }

    pub fn weight(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.weight.get(ruby)
    }
}

impl DataTypeFunctions for YFontFaceRule {
    fn mark(&self, marker: &gc::Marker) {
        self.ascent_override.mark(marker);
        self.descent_override.mark(marker);
        self.display.mark(marker);
        self.family.mark(marker);

        if let Some(font_face) = &self.font_face {
            font_face.mark(marker);
        }

        self.line_gap_override.mark(marker);
        self.size_adjust.mark(marker);

        if let Some(sources) = &self.sources {
            sources.mark(marker);
        }

        self.font_stretch_range.mark(marker);
        self.style.mark(marker);

        if let Some(unicode_range) = &self.unicode_range {
            unicode_range.mark(marker);
        }

        self.weight.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::FontFace", mark)]
pub struct YFontFace {
    family: CachedValue<FamilyName>,
    sources: CachedValueList<Source>,
}

impl YFontFace {
    pub fn new(family: FamilyName, sources: SourceList) -> Self {
        Self {
            family: CachedValue::new(family, |family, ruby| {
                YFontFamilyName::new(family.clone()).into_value_with(ruby)
            }),

            sources: CachedValueList::new(sources.0.to_vec(), |source, _ctx, ruby| {
                font_source_to_value(source, ruby)
            })
        }
    }

    pub fn family(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.family.get(ruby)
    }

    pub fn sources(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        rb_self.sources.to_a(ruby)
    }
}

impl DataTypeFunctions for YFontFace {
    fn mark(&self, marker: &gc::Marker) {
        self.family.mark(marker);
        self.sources.mark(marker);
    }
}
