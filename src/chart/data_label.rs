use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::CHART_NS",
    name = "data-label"
)]
#[derive(Default)]
pub struct DataLabel {}
