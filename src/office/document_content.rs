use super::font_face_decls::FontFaceDecls;
use super::automatic_styles::AutomaticStyles;
use super::body::Body;
use super::scripts::Scripts;
use std::str::FromStr;
use odf_macros::define_element;

#[define_element(
    setup_prefixes="true",
    namespace = "crate::ns::OFFICE_NS",
    name = "automatic-styles",
    attribute(name="transformation", r#type="String", prefix="grddl"),
    attribute(name="version", r#type="String", prefix="office"),
    child(r#type="AutomaticStyles"),
    child(r#type="Body"),
    child(r#type="FontFaceDecls"),
    child(r#type="Scripts")
)]
#[derive(Default)]
pub struct DocumentContent {}