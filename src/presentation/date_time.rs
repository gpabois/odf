use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::PRESENTATION_NS",
    name = "date-time"
)]
#[derive(Default)]
pub struct DateTime {}
