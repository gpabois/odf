use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::TEXT_NS",
    name = "creator"
)]
#[derive(Default)]
pub struct Creator {}
