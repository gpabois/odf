use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::TEXT_NS",
    name = "ruby"
)]
#[derive(Default)]
pub struct Ruby {}