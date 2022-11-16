use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::TEXT_NS",
    name = "outline-style"
)]
#[derive(Default)]
pub struct OutlineStyle {}
