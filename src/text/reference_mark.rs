use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::TEXT_NS",
    name = "reference-mark"
)]
#[derive(Default)]
pub struct ReferenceMark {}