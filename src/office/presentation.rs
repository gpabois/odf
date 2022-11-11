use crate::element::OpenDocumentElement;
use odf_macros::define_element;
use crate::ns::OFFICE_NS;

#[define_element(
    namespace = "OFFICE_NS",
    name = "presentation"
)]
#[derive(Default)]
pub struct Presentation{}