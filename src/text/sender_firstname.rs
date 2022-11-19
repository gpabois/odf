use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::TEXT_NS",
    name = "sender-firstname"
)]
#[derive(Default)]
pub struct SenderFirstname {}