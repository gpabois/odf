use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::DRAWING_NS",
    name = "control"
)]
#[derive(Default)]
pub struct Control {}