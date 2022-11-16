use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::TEXT_NS",
    name = "soft-page-break"
)]

#[derive(Default)]
pub struct SoftPageBreak {}