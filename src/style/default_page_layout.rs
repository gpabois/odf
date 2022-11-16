use odf_macros::{ define_element};


#[define_element(
    namespace = "crate::ns::STYLE_NS",
    name = "default-page-layout",
)]
#[derive(Default)]
pub struct DefaultPageLayout {}