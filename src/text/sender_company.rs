use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::TEXT_NS",
    name = "sender-company"
)]
#[derive(Default)]
pub struct SenderCompany {}