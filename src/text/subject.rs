use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::TEXT_NS",
    name = "subject"
)]

#[derive(Default)]
pub struct Subject {}