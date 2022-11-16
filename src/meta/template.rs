use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::META_NS",
    name = "template"
)]
#[derive(Default)]
pub struct Template {}