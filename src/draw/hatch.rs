use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::DRAWING_NS",
    name = "hatch"
)]
#[derive(Default)]
pub struct Hatch {}