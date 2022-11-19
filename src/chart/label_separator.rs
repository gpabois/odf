use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::CHART_NS",
    name = "label_separator"
)]
#[derive(Default)]
pub struct LabelSeperator {}
