use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::META_NS",
    name = "printed-by"
)]
#[derive(Default)]
pub struct PrintedBy {}