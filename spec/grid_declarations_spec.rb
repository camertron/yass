# frozen_string_literal: true

require "spec_helper"

RSpec.describe "grid declarations" do
  def declaration_for(property, value)
    sheet = Yass::Parser.parse(".x { #{property}: #{value}; }")
    sheet.rules.first.declarations.first
  end

  it "exposes grid-auto-columns values" do
    declaration = declaration_for("grid-auto-columns", "100px minmax(10px, 1fr)")

    expect(declaration).to be_a(Yass::Declarations::GridAutoColumns)

    values = declaration.values
    expect(values.size).to eq(2)

    track_breadth = values[0]
    expect(track_breadth).to be_a(Yass::Declarations::TrackBreadth::LengthPercentage)

    expect(track_breadth.value).to be_a(Yass::Declarations::Length::Absolute)
    expect(track_breadth.value.value).to eq(100.0)
    expect(track_breadth.value.unit).to eq(:px)

    track_size = values[1]
    expect(track_size).to be_a(Yass::Declarations::TrackSize::Minmax)

    expect(track_size.min).to be_a(Yass::Declarations::TrackBreadth::LengthPercentage)
    expect(track_size.min.value.value).to eq(10.0)
    expect(track_size.min.value.unit).to eq(:px)

    expect(track_size.max).to be_a(Yass::Declarations::TrackBreadth::Fr)
    expect(track_size.max.value).to eq(1.0)
  end

  it "exposes grid-auto-flow axis and dense flag" do
    declaration = declaration_for("grid-auto-flow", "column dense")

    expect(declaration).to be_a(Yass::Declarations::GridAutoFlow)
    expect(declaration.value).to eq(:column)
    expect(declaration.axis).to eq(:column)
    expect(declaration).to be_dense
  end

  it "exposes grid-auto-rows values" do
    declaration = declaration_for("grid-auto-rows", "fit-content(20%)")

    expect(declaration).to be_a(Yass::Declarations::GridAutoRows)

    values = declaration.values
    expect(values.size).to eq(1)

    track_size = values[0]
    expect(track_size).to be_a(Yass::Declarations::TrackSize::FitContent)

    breadth = track_size.value
    expect(breadth).to be_a(Yass::Declarations::TrackBreadth::LengthPercentage)
    expect(breadth.value).to be_a(Yass::Declarations::Percentage)
  end

  it "exposes grid line declarations" do
    start_declaration = declaration_for("grid-column-start", "span 2 nav")
    end_declaration = declaration_for("grid-column-end", "5")
    row_start_declaration = declaration_for("grid-row-start", "content")
    row_end_declaration = declaration_for("grid-row-end", "auto")

    expect(start_declaration).to be_a(Yass::Declarations::GridColumnStart)
    expect(start_declaration.ident).to eq("nav")
    expect(start_declaration.line_num).to eq(2)
    expect(start_declaration).to be_span
    expect(start_declaration).not_to be_auto

    expect(end_declaration).to be_a(Yass::Declarations::GridColumnEnd)
    expect(end_declaration.line_num).to eq(5)
    expect(end_declaration.ident).to be_nil

    expect(row_start_declaration).to be_a(Yass::Declarations::GridRowStart)
    expect(row_start_declaration.ident).to eq("content")
    expect(row_start_declaration).to be_ident_only

    expect(row_end_declaration).to be_a(Yass::Declarations::GridRowEnd)
    expect(row_end_declaration).to be_auto
  end

  it "exposes grid-template-areas variants" do
    none_declaration = declaration_for("grid-template-areas", "none")
    expect(none_declaration).to be_a(Yass::Declarations::GridTemplateAreas)
    expect(none_declaration.areas).to be_a(Yass::Declarations::GridTemplateAreas::None)

    areas_declaration = declaration_for("grid-template-areas", '"a a" "b c"')
    area_list = areas_declaration.areas

    expect(area_list).to be_a(Yass::Declarations::GridTemplateAreas::AreaList)
    expect(area_list.width).to eq(2)
    expect(area_list.strings).to eq(["a a", "b c"])

    first_area = area_list.areas.first
    expect(first_area).to be_a(Yass::Declarations::GridTemplateAreas::Area)
    expect(first_area.name).to eq("a")
  end

  it "exposes grid-template-columns structure" do
    declaration = declaration_for("grid-template-columns", "[left] 100px repeat(2, [col] 1fr) [right]")

    expect(declaration).to be_a(Yass::Declarations::GridTemplateColumns)

    track_list = declaration.value
    expect(track_list).to be_a(Yass::Declarations::GridTemplate::TrackList)
    expect(track_list.values.size).to eq(2)

    repeat_value = track_list.values[1].value
    expect(repeat_value).to be_a(Yass::Declarations::GridTemplate::TrackListValue::TrackRepeat)

    count = repeat_value.count
    expect(count).to be_a(Yass::Declarations::GridTemplate::RepeatCount::Number)
    expect(count.value).to eq(2)
  end

  it "exposes grid-template-rows structure" do
    declaration = declaration_for("grid-template-rows", "[top] minmax(10px, 1fr) [bottom]")

    expect(declaration).to be_a(Yass::Declarations::GridTemplateRows)

    track_list = declaration.value
    expect(track_list).to be_a(Yass::Declarations::GridTemplate::TrackList)

    first_value = track_list.values.first.value
    expect(first_value).to be_a(Yass::Declarations::TrackSize::Minmax)
  end
end
