
use odf_macros::{define_element, define_child_elements};
use crate::table;
use crate::text;
use std::str::FromStr;

#[define_element(
    namespace = "crate::ns::OFFICE_NS",
    name = "spreadsheet",
    children = "SpreadsheetChildElement",
    attribute(name = "protection-key", r#type="String", prefix = "table"),
    attribute(name = "protection-key-digest-algorithm", r#type="String", prefix = "table"),
    attribute(name = "structure-protected", r#type="String", prefix = "table")
)]
#[derive(Default)]
pub struct Spreadsheet {}

#[define_child_elements(
    table::CalculationSettings,
    table::Consolidation,
    table::ContentValidations,
    table::DataPilotTables,
    table::DatabaseRanges,
    table::DdeLinks,
    table::TrackedChanges,
    table::LabelRanges,
    table::NamedExpressions,
    table::Table,
    text::AlphabeticalIndexAutoMarkFile,
    text::DdeConnectionDecls,
    text::SequenceDecls,
    text::UserFieldDecls,
    text::VariableDecls
)]
pub enum SpreadsheetChildElement {}