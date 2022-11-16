use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::TEXT_NS",
    name = "table-of-content"
)]
#[derive(Default)]
pub struct TableOfContent {}