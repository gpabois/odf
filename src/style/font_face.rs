use odf_macros::{ define_element};


#[define_element(
    namespace = "crate::ns::STYLE_NS",
    name = "font-face",
)]
#[derive(Default)]
pub struct FontFace {}