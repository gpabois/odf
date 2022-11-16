
use odf_macros::{define_element, define_child_elements};
use crate::chart;
use crate::table;
use crate::text;

#[define_element(
    namespace = "crate::ns::OFFICE_NS",
    name = "chart"
)]
#[derive(Default)]
pub struct Chart {}

#[define_child_elements(
    chart::Chart,
    table::CalculationSettings,
    table::Consolidation,
    table::ContentValidations,
    table::DataPilotTables,
    table::DatabaseRanges,
    table::DdeLinks,
    table::LabelRanges,
    table::NamedExpressions,
    text::AlphabeticalIndexAutoMarkFile,
    text::DdeConnectionDecls,
    text::SequenceDecls,
    text::UserFieldDecls,
    text::VariableDecls
)]
pub enum ChartChildElement {}