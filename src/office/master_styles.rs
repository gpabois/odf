use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::OFFICE_NS",
    name = "master-styles"
)]
#[derive(Default)]
pub struct MasterStyles {}