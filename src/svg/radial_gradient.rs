use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::PRESENTATION_NS",
    name = "radientGradient"
)]
#[derive(Default)]
pub struct RadialGradient {}
