use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::TEXT_NS",
    name = "page-count"
)]
#[derive(Default)]
pub struct PageCount {}