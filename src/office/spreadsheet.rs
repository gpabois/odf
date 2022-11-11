use crate::element::OpenDocumentElement;
use odf_macros::define_element;
use crate::ns::OFFICE_NS;

#[define_element(
    namespace = "OFFICE_NS",
    name = "spreadsheet"
)]
#[derive(Default)]
pub struct Spreadsheet{}