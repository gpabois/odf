use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::TEXT_NS",
    name = "a"
)]
#[derive(Default)]
pub struct A {}
