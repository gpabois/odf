use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::TEXT_NS",
    name = "reference-mark-start"
)]
#[derive(Default)]
pub struct ReferenceMarkStart {}