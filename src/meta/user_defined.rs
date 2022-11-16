use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::META_NS",
    name = "user-defined"
)]
#[derive(Default)]
pub struct UserDefined {}