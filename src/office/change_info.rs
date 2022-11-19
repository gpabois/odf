use odf_macros::{define_element};


#[define_element(
    namespace = "crate::ns::OFFICE_NS",
    name = "change-info"
)]
#[derive(Default)]
pub struct ChangeInfo {}
