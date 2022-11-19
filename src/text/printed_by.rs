use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::TEXT_NS",
    name = "printed-by"
)]
#[derive(Default)]
pub struct PrintedBy {}