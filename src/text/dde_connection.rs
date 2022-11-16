use odf_macros::define_element;


#[define_element(
    namespace = "crate::ns::TEXT_NS",
    name = "dde-connection"
)]
#[derive(Default)]
pub struct DdeConnection {}