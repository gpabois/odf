use crate::{table::{Table}, text::{AlphabeticalIndex, AlphabeticalIndexAutoMarkFile, Bibliography, Change, ChangeEnd, ChangeStart, DdeConnectionDecls, H, IllustrationIndex, List, NumberedParagraph, ObjectIndex, P, PageSequence, Section, SequenceDecls, SoftPageBreak, TableIndex, TableOfContent, TrackedChanges, UserFieldDecls, VariableDecls, UserIndex}};
use crate::{dr3d};
use crate::draw;
use crate::table;
use crate::ns::OFFICE_NS;


use odf_macros::{define_child_elements, define_element};

#[define_element(
    namespace = "OFFICE_NS",
    name = "text",
    children = "TextChildElement",
    attribute(name="global", r#type="String", prefix="bool"),
    attribute(name="use-soft-page-breaks", r#type="String", prefix="bool"),
)]
#[derive(Default)]
pub struct Text {}

impl Text {
    pub fn add(&mut self, child: impl Into<TextChildElement>) {
        self.children.push(child.into());
    }
}

#[define_child_elements(
    dr3d::Scene,
    draw::A, draw::Caption, draw::Circle, draw::Connector,
    draw::Control, draw::CustomShape, draw::Ellipse, draw::Frame,
    draw::G, draw::Line, draw::Measure, draw::PageThumbnail, draw::Path,
    draw::Polygon, draw::Polyline, draw::Rect, draw::RegularPolygon,
    table::CalculationSettings, table::Consolidation, table::ContentValidations,
    table::DataPilotTables, table::DatabaseRanges, table::DdeLinks,
    table::LabelRanges, table::NamedExpressions, Table,
    AlphabeticalIndex, AlphabeticalIndexAutoMarkFile, 
    Bibliography,
    Change, ChangeEnd, ChangeStart,
    DdeConnectionDecls,
    H, IllustrationIndex, List, NumberedParagraph, ObjectIndex,
    P, PageSequence, Section, SequenceDecls, SoftPageBreak,
    TableIndex, TableOfContent, TrackedChanges, UserFieldDecls, UserIndex, VariableDecls)]
pub enum TextChildElement
{
    // Non implémenté: office:forms
}