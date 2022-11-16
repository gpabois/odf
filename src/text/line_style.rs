use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::TEXT_NS",
    name = "list-style"
)]
#[derive(Default)]
pub struct ListStyle {}