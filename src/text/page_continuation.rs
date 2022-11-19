use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::TEXT_NS",
    name = "page-continuation"
)]
#[derive(Default)]
pub struct PageContinuation {}