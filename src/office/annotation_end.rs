use odf_macros::{define_element};


#[define_element(
    namespace = "crate::ns::OFFICE_NS",
    name = "annotation-end"
)]
#[derive(Default)]
pub struct AnnotationEnd {}
