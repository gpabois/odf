use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::META_NS",
    name = "document-statistic"
)]
#[derive(Default)]
pub struct DocumentStatistic {}