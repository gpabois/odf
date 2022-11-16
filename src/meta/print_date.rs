use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::META_NS",
    name = "print-date"
)]
#[derive(Default)]
pub struct PrintDate {}