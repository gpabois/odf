use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::DRAWING_NS",
    name = "rect"
)]
#[derive(Default)]
pub struct Rect {}