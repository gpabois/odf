use odf_macros::{ define_element};


#[define_element(
    namespace = "crate::ns::STYLE_NS",
    name = "default-style",
)]
#[derive(Default)]
pub struct DefaultStyle {}