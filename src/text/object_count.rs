use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::TEXT_NS",
    name = "object-count"
)]

#[derive(Default)]
pub struct ObjectCount {}