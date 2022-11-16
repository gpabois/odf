use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::CHART_NS",
    name = "chart"
)]
#[derive(Default)]
pub struct Chart {}
