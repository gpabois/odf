use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::CHART_NS",
    name = "title"
)]
#[derive(Default)]
pub struct Title {}
