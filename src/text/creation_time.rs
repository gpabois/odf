use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::TEXT_NS",
    name = "creation-time"
)]
#[derive(Default)]
pub struct CreationTime {}
