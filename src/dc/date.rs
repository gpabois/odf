use odf_macros::define_element;


#[define_element(
    namespace = "crate::ns::DC_NS",
    name = "date"
)]
#[derive(Default)]
pub struct Date {}