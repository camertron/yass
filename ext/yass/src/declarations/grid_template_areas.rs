use magnus::{DataTypeFunctions, Error, IntoValue, RArray, Ruby, TypedData, Value, gc, typed_data};
use style::values::{computed::GridTemplateAreas, specified::position::{NamedArea, TemplateAreasArc}};

use crate::{cached_value::CachedValue, cached_value_list::CachedValueList};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::GridTemplateAreas", mark)]
pub struct YGridTemplateAreas {
    areas: CachedValue<GridTemplateAreas>,
}

impl YGridTemplateAreas {
    pub fn new(grid_template_areas: GridTemplateAreas) -> Self {
        Self {
            areas: CachedValue::new(grid_template_areas, make_grid_template_areas_value),
        }
    }

    pub fn areas(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.areas.get(ruby)
    }
}

impl DataTypeFunctions for YGridTemplateAreas {
    fn mark(&self, marker: &gc::Marker) {
        self.areas.mark(marker);
    }
}

fn make_grid_template_areas_value(grid_template_areas: &GridTemplateAreas, ruby: &Ruby) -> Value {
    match grid_template_areas {
        GridTemplateAreas::None => YGridTemplateAreasNone::new().into_value_with(ruby),
        GridTemplateAreas::Areas(areas) => YGridTemplateAreaList::new(areas.clone()).into_value_with(ruby),
    }
}

#[magnus::wrap(class = "Yass::Declarations::GridTemplateAreas::None")]
pub struct YGridTemplateAreasNone {}

impl YGridTemplateAreasNone {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::GridTemplateAreas::AreaList", mark)]
pub struct YGridTemplateAreaList {
    width: u32,
    strings: CachedValueList<String>,
    areas: CachedValueList<NamedArea>,
}

impl YGridTemplateAreaList {
    pub fn new(template_areas: TemplateAreasArc) -> Self {
        let areas = &template_areas.0;

        Self {
            width: areas.width,

            strings: CachedValueList::new(
                areas.strings.iter().map(|s| s.to_string()).collect(),
                |value, _ctx, ruby| ruby.str_new(value).into_value_with(ruby),
            ),

            areas: CachedValueList::new(areas.areas.to_vec(), |value, _ctx, ruby| {
                YGridTemplateArea::new(value.clone()).into_value_with(ruby)
            }),
        }
    }

    pub fn width(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> u32 {
        rb_self.width
    }

    pub fn strings(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        rb_self.strings.to_a(ruby)
    }

    pub fn areas(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        rb_self.areas.to_a(ruby)
    }
}

impl DataTypeFunctions for YGridTemplateAreaList {
    fn mark(&self, marker: &gc::Marker) {
        self.strings.mark(marker);
        self.areas.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::Declarations::GridTemplateAreas::Area")]
pub struct YGridTemplateArea {
    name: String,
    row_start: u32,
    row_end: u32,
    column_start: u32,
    column_end: u32,
}

impl YGridTemplateArea {
    pub fn new(area: NamedArea) -> Self {
        Self {
            name: area.name.to_string(),
            row_start: area.rows.start,
            row_end: area.rows.end,
            column_start: area.columns.start,
            column_end: area.columns.end,
        }
    }

    pub fn name(ruby: &Ruby, rb_self: &Self) -> Value {
        ruby.str_new(&rb_self.name).into_value_with(ruby)
    }

    pub fn row_start(_ruby: &Ruby, rb_self: &Self) -> u32 {
        rb_self.row_start
    }

    pub fn row_end(_ruby: &Ruby, rb_self: &Self) -> u32 {
        rb_self.row_end
    }

    pub fn column_start(_ruby: &Ruby, rb_self: &Self) -> u32 {
        rb_self.column_start
    }

    pub fn column_end(_ruby: &Ruby, rb_self: &Self) -> u32 {
        rb_self.column_end
    }
}
