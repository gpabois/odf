use super::font_face_decls::FontFaceDecls;
use super::automatic_styles::AutomaticStyles;
use super::body::Body;
use super::scripts::Scripts;

#[derive(Default)]
pub struct DocumentContent
{
    automatic_styles: AutomaticStyles,
    body: Body,
    font_face_decls: FontFaceDecls,
    scripts: Scripts
}