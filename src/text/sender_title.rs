use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::TEXT_NS",
    name = "sender-title"
)]
#[derive(Default)]
pub struct SenderTitle {}