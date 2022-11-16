use odf_macros::define_element;


#[define_element(
    namespace = "crate::ns::DC_NS",
    name = "description"
)]
#[derive(Default)]
pub struct Description {}