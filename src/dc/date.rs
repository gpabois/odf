use odf_macros::define_element;


#[define_element(
    namespace = "crate::ns::DC_NS",
    name = "date",
    content = "String"
)]
#[derive(Default)]
pub struct Date {}