use odf_macros::define_child_elements;
use odf_macros::define_element;

use crate::table;
use crate::draw;
use crate::text;

#[define_element(
    namespace = "crate::ns::OFFICE_NS",
    name = "drawing",
    children = "DrawingChildElement"
)]
#[derive(Default)]
pub struct Drawing{}

#[define_child_elements(
    draw::Page,
    table::CalculationSettings,
    table::Consolidation,
    table::ContentValidations,
    table::DataPilotTables,
    table::DatabaseRanges,
    table::DdeLinks,
    table::LabelRanges,
    table::NamedExpressions,
    text::AlphabeticalIndex,
    text::DdeConnectionDecls,
    text::SequenceDecls,
    text::UserFieldDecls,
    text::VariableDecls
)]
pub enum DrawingChildElement {}