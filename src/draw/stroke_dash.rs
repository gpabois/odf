use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::DRAWING_NS",
    name = "stroke-dash"
)]
#[derive(Default)]
pub struct StrokeDash {}