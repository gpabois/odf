use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::DRAWING_NS",
    name = "path"
)]
#[derive(Default)]
pub struct Path {}