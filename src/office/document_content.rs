use super::font_face_decls::FontFaceDecls;
use super::automatic_styles::AutomaticStyles;
use super::body::Body;
use super::scripts::Scripts;

use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::OFFICE_NS",
    name = "automatic-styles",
    child(r#type="AutomaticStyles"),
    child(r#type="Body"),
    child(r#type="FontFaceDecls"),
    child(r#type="Scripts")
)]
#[derive(Default)]
pub struct DocumentContent
{
    //automatic_styles: AutomaticStyles,
    //body: Body,
    //font_face_decls: FontFaceDecls,
    //scripts: Scripts
}