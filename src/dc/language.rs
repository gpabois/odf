use odf_macros::define_element;


#[define_element(
    namespace = "crate::ns::DC_NS",
    name = "language",
    content = "String"
)]
#[derive(Default)]
pub struct Language {}