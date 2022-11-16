use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::TEXT_NS",
    name = "chapter"
)]
#[derive(Default)]
pub struct Chapter {}
