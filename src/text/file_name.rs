use odf_macros::define_element;


#[define_element(
    namespace = "crate::ns::TEXT_NS",
    name = "file-name"
)]
#[derive(Default)]
pub struct FileName {}