use magnus::{DataTypeFunctions, Error, IntoValue, RArray, Ruby, TypedData, Value, gc, typed_data};
use style::{OwnedSlice, values::{
    CustomIdent, generics::grid::{LineNameList, LineNameListValue, NameRepeat, RepeatCount, TrackListValue, TrackRepeat}, specified::{GridTemplateComponent, Integer, LengthPercentage, TrackList, TrackSize}
}};

use crate::{
    cached_value::CachedValue,
    cached_value_list::CachedValueList, declarations::track_size::make_track_size_value,
};

pub fn make_grid_template_component_value(component: &GridTemplateComponent, ruby: &Ruby) -> Value {
    match component {
        GridTemplateComponent::None => YGridTemplateNone::new().into_value_with(ruby),
        GridTemplateComponent::Masonry => YGridTemplateMasonry::new().into_value_with(ruby),
        GridTemplateComponent::TrackList(track_list) => {
            YGridTemplateTrackList::new((**track_list).clone()).into_value_with(ruby)
        }
        GridTemplateComponent::Subgrid(line_name_list) => {
            YGridTemplateSubgrid::new((**line_name_list).clone()).into_value_with(ruby)
        }
    }
}

#[magnus::wrap(class = "Yass::Declarations::GridTemplate::None")]
pub struct YGridTemplateNone {}

impl YGridTemplateNone {
    pub fn new() -> Self {
        Self {}
    }
}

#[magnus::wrap(class = "Yass::Declarations::GridTemplate::Masonry")]
pub struct YGridTemplateMasonry {}

impl YGridTemplateMasonry {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::GridTemplate::TrackList", mark)]
pub struct YGridTemplateTrackList {
    auto_repeat_index: usize,
    explicit: bool,
    auto_repeat: bool,
    values: CachedValueList<TrackListValue<LengthPercentage, Integer>>,
    line_names: CachedValueList<Vec<String>>,
}

impl YGridTemplateTrackList {
    pub fn new(track_list: TrackList) -> Self {
        Self {
            auto_repeat_index: track_list.auto_repeat_index,
            explicit: track_list.is_explicit(),
            auto_repeat: track_list.has_auto_repeat(),

            values: CachedValueList::new(track_list.values.to_vec(), |value, _ctx, ruby| {
                YGridTemplateTrackListValue::new(value.clone()).into_value_with(ruby)
            }),

            line_names: CachedValueList::new(track_list.line_names.iter().map(idents_to_strings).collect(), |value, _ctx, ruby| {
                strings_to_array(value, ruby)
            }),
        }
    }

    pub fn auto_repeat_index(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> usize {
        rb_self.auto_repeat_index
    }

    pub fn explicit(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.explicit
    }

    pub fn auto_repeat(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.auto_repeat
    }

    pub fn values(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        rb_self.values.to_a(ruby)
    }

    pub fn line_names(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        rb_self.line_names.to_a(ruby)
    }
}

impl DataTypeFunctions for YGridTemplateTrackList {
    fn mark(&self, marker: &gc::Marker) {
        self.values.mark(marker);
        self.line_names.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::GridTemplate::TrackListValue", mark)]
pub struct YGridTemplateTrackListValue {
    value: CachedValue<TrackListValue<LengthPercentage, Integer>>,
}

impl YGridTemplateTrackListValue {
    pub fn new(track_list_value: TrackListValue<LengthPercentage, Integer>) -> Self {
        Self {
            value: CachedValue::new(track_list_value, make_track_list_value),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.value.get(ruby)
    }
}

impl DataTypeFunctions for YGridTemplateTrackListValue {
    fn mark(&self, marker: &gc::Marker) {
        self.value.mark(marker);
    }
}

fn make_track_list_value(track_list_value: &TrackListValue<LengthPercentage, Integer>, ruby: &Ruby) -> Value {
    match track_list_value {
        TrackListValue::TrackSize(track_size) => make_track_size_value(track_size, ruby),
        TrackListValue::TrackRepeat(track_repeat) => {
            YGridTemplateTrackListValueTrackRepeat::new(track_repeat.clone())
                .into_value_with(ruby)
        }
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::GridTemplate::TrackListValue::TrackRepeat", mark)]
pub struct YGridTemplateTrackListValueTrackRepeat {
    count: CachedValue<RepeatCount<Integer>>,
    line_names: CachedValueList<Vec<String>>,
    track_sizes: CachedValueList<TrackSize>,
}

impl YGridTemplateTrackListValueTrackRepeat {
    pub fn new(track_repeat: TrackRepeat<LengthPercentage, Integer>) -> Self {
        Self {
            count: CachedValue::new(track_repeat.count, make_repeat_count_value),

            line_names: CachedValueList::new(track_repeat.line_names.iter().map(idents_to_strings).collect(), |value, _ctx, ruby| {
                strings_to_array(value, ruby)
            }),

            track_sizes: CachedValueList::new(track_repeat.track_sizes.to_vec(), |value, _ctx, ruby| {
                make_track_size_value(value, ruby)
            }),
        }
    }

    pub fn count(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.count.get(ruby)
    }

    pub fn line_names(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        rb_self.line_names.to_a(ruby)
    }

    pub fn track_sizes(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        rb_self.track_sizes.to_a(ruby)
    }
}

impl DataTypeFunctions for YGridTemplateTrackListValueTrackRepeat {
    fn mark(&self, marker: &gc::Marker) {
        self.count.mark(marker);
        self.line_names.mark(marker);
        self.track_sizes.mark(marker);
    }
}

fn make_repeat_count_value(count: &RepeatCount<Integer>, ruby: &Ruby) -> Value {
    match count {
        RepeatCount::Number(value) => YGridTemplateRepeatCountNumber::new(value.value()).into_value_with(ruby),
        RepeatCount::AutoFill => YGridTemplateRepeatCountAutoFill::new().into_value_with(ruby),
        RepeatCount::AutoFit => YGridTemplateRepeatCountAutoFit::new().into_value_with(ruby),
    }
}

#[magnus::wrap(class = "Yass::Declarations::GridTemplate::RepeatCount::Number")]
pub struct YGridTemplateRepeatCountNumber {
    value: i32,
}

impl YGridTemplateRepeatCountNumber {
    pub fn new(value: i32) -> Self {
        Self { value }
    }

    pub fn value(_ruby: &Ruby, rb_self: &Self) -> i32 {
        rb_self.value
    }
}

#[magnus::wrap(class = "Yass::Declarations::GridTemplate::RepeatCount::AutoFill")]
pub struct YGridTemplateRepeatCountAutoFill {}

impl YGridTemplateRepeatCountAutoFill {
    pub fn new() -> Self {
        Self {}
    }
}

#[magnus::wrap(class = "Yass::Declarations::GridTemplate::RepeatCount::AutoFit")]
pub struct YGridTemplateRepeatCountAutoFit {}

impl YGridTemplateRepeatCountAutoFit {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::GridTemplate::Subgrid", mark)]
pub struct YGridTemplateSubgrid {
    expanded_line_names_length: usize,
    line_names: CachedValueList<LineNameListValue<Integer>>,
}

impl YGridTemplateSubgrid {
    pub fn new(line_name_list: LineNameList<Integer>) -> Self {
        Self {
            expanded_line_names_length: line_name_list.expanded_line_names_length,
            line_names: CachedValueList::new(line_name_list.line_names.to_vec(), |value, _ctx, ruby| {
                YGridTemplateLineNameListValue::new(value.clone()).into_value_with(ruby)
            }),
        }
    }

    pub fn expanded_line_names_length(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> usize {
        rb_self.expanded_line_names_length
    }

    pub fn line_names(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        rb_self.line_names.to_a(ruby)
    }
}

impl DataTypeFunctions for YGridTemplateSubgrid {
    fn mark(&self, marker: &gc::Marker) {
        self.line_names.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::GridTemplate::LineNameListValue", mark)]
pub struct YGridTemplateLineNameListValue {
    value: CachedValue<LineNameListValue<Integer>>,
}

impl YGridTemplateLineNameListValue {
    pub fn new(line_name_list_value: LineNameListValue<Integer>) -> Self {
        Self {
            value: CachedValue::new(line_name_list_value, make_line_name_list_value),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.value.get(ruby)
    }
}

impl DataTypeFunctions for YGridTemplateLineNameListValue {
    fn mark(&self, marker: &gc::Marker) {
        self.value.mark(marker);
    }
}

fn make_line_name_list_value(line_name_list_value: &LineNameListValue<Integer>, ruby: &Ruby) -> Value {
    match line_name_list_value {
        LineNameListValue::LineNames(names) => {
            YGridTemplateLineNames::new(idents_to_strings(names)).into_value_with(ruby)
        }
        LineNameListValue::Repeat(repeat) => {
            YGridTemplateLineNameRepeat::new(repeat.clone()).into_value_with(ruby)
        }
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::GridTemplate::LineNames", mark)]
pub struct YGridTemplateLineNames {
    names: CachedValueList<String>,
}

impl YGridTemplateLineNames {
    pub fn new(names: Vec<String>) -> Self {
        Self {
            names: CachedValueList::new(names, |value, _ctx, ruby| {
                ruby.str_new(value).into_value_with(ruby)
            }),
        }
    }

    pub fn names(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        rb_self.names.to_a(ruby)
    }
}

impl DataTypeFunctions for YGridTemplateLineNames {
    fn mark(&self, marker: &gc::Marker) {
        self.names.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::GridTemplate::LineNameRepeat", mark)]
pub struct YGridTemplateLineNameRepeat {
    count: CachedValue<RepeatCount<Integer>>,
    line_names: CachedValueList<Vec<String>>,
}

impl YGridTemplateLineNameRepeat {
    pub fn new(repeat: NameRepeat<Integer>) -> Self {
        Self {
            count: CachedValue::new(repeat.count, make_repeat_count_value),

            line_names: CachedValueList::new(repeat.line_names.iter().map(idents_to_strings).collect(), |value, _ctx, ruby| {
                strings_to_array(value, ruby)
            }),
        }
    }

    pub fn count(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.count.get(ruby)
    }

    pub fn line_names(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        rb_self.line_names.to_a(ruby)
    }
}

impl DataTypeFunctions for YGridTemplateLineNameRepeat {
    fn mark(&self, marker: &gc::Marker) {
        self.count.mark(marker);
        self.line_names.mark(marker);
    }
}

fn idents_to_strings(names: &OwnedSlice<CustomIdent>) -> Vec<String> {
    names.iter().map(|name| name.0.to_string()).collect()
}

fn strings_to_array(values: &[String], ruby: &Ruby) -> Value {
    let array = ruby.ary_new_capa(values.len());

    for value in values {
        let _ = array.push(ruby.str_new(value));
    }

    array.into_value_with(ruby)
}
