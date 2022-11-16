use odf_macros::{ define_element};


#[define_element(
    namespace = "crate::ns::STYLE_NS",
    name = "style",
)]
#[derive(Default)]
pub struct Style {}