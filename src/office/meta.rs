use crate::{dc::{Creator, Date, Description, Subject, Language}, meta::{AutoReload, CreationDate, DocumentStatistic, EditingCycles, EditingDuration, Generator, HyperlinkBehaviour, InitialCreator, PrintDate, PrintedBy, Template, UserDefined, Keyword}};
use odf_macros::{define_element, define_child_elements};

#[define_element(
    namespace = "crate::ns::OFFICE_NS",
    name="meta",
    children="MetaChildElement"
)]
#[derive(Default)]
pub struct Meta {
    // children: Vec<MetaChildElement>
}

#[define_child_elements(
    Creator, Date, Description, Subject,
    Language, AutoReload, CreationDate,
    DocumentStatistic, EditingCycles,
    EditingDuration, Generator,
    HyperlinkBehaviour, InitialCreator,
    Keyword, PrintDate, PrintedBy, Template, UserDefined
)]
pub enum MetaChildElement {}