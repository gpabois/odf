use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::TEXT_NS",
    name = "page-sequence"
)]
#[derive(Default)]
pub struct PageSequence {}