use odf_macros::define_element;
use crate::element::OpenDocumentElement;

use crate::ns::OFFICE_NS;

#[define_element(
    namespace = "OFFICE_NS",
    name = "database"
)]
#[derive(Default)]
pub struct Database{}