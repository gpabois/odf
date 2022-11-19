use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::TEXT_NS",
    name = "sender-fax"
)]
#[derive(Default)]
pub struct SenderFax {}