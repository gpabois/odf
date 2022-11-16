use odf_macros::define_element;


#[define_element(
    namespace = "crate::ns::TEXT_NS",
    name = "change-start"
)]
#[derive(Default)]
pub struct ChangeStart {}
