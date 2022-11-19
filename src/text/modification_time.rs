use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::TEXT_NS",
    name = "modification-time"
)]
#[derive(Default)]
pub struct ModificationTime {}