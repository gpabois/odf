
use odf_macros::define_child_elements;
use odf_macros::define_element;

use crate::presentation;
use crate::draw;
use crate::table;
use crate::text;

#[define_element(
    namespace = "crate::ns::OFFICE_NS",
    name = "presentation",
    children = "PresentationChildElement"
)]
#[derive(Default)]
pub struct Presentation{}

#[define_child_elements(
    draw::Page,
    presentation::DateTimeDecl,
    presentation::HeaderDecl,
    presentation::FooterDecl,
    presentation::Settings,
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
pub enum PresentationChildElement {

}