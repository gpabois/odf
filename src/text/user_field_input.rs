use odf_macros::define_element;


#[define_element(
    namespace = "crate::ns::TEXT_NS",
    name = "user-field-input"
)]
#[derive(Default)]
pub struct UserFieldInput {}