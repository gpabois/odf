use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::TEXT_NS",
    name = "change"
)]
#[derive(Default)]
pub struct Change {}
