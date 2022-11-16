use odf_macros::{ define_element};
use crate::ns::OFFICE_NS;

#[define_element(
    namespace = "OFFICE_NS",
    name = "font-face-decls",
)]
#[derive(Default)]
pub struct FontFaceDecls {}