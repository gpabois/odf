use odf_macros::{ define_element};


#[define_element(
    namespace = "crate::ns::STYLE_NS",
    name = "page-layout",
)]
#[derive(Default)]
pub struct PageLayout {}