
use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::OFFICE_NS",
    name = "presentation"
)]
#[derive(Default)]
pub struct Presentation{}