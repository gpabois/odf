
use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::OFFICE_NS",
    name = "spreadsheet"
)]
#[derive(Default)]
pub struct Spreadsheet{}