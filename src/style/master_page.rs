use odf_macros::{ define_element};


#[define_element(
    namespace = "crate::ns::STYLE_NS",
    name = "master-page",
)]
#[derive(Default)]
pub struct MasterPage {}