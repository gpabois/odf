use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::META_NS",
    name = "generator"
)]
#[derive(Default)]
pub struct Generator {}