use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::PRESENTATION_NS",
    name = "settings"
)]
#[derive(Default)]
pub struct Settings {}
