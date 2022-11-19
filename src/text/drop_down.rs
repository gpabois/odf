use odf_macros::define_element;


#[define_element(
    namespace = "crate::ns::TEXT_NS",
    name = "drop-down"
)]
#[derive(Default)]
pub struct DropDown {}