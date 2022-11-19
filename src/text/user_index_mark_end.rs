use odf_macros::define_element;


#[define_element(
    namespace = "crate::ns::TEXT_NS",
    name = "user-index-mark-end"
)]
#[derive(Default)]
pub struct UserIndexMarkEnd {}