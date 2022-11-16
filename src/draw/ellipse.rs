use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::DRAWING_NS",
    name = "ellipse"
)]
#[derive(Default)]
pub struct Ellipse {}