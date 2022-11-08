pub static OFFICE_NS: &'static str = "urn:oasis:names:tc:opendocument:xmlns:office:1.0";
pub static META_NS: &'static str = "urn:oasis:names:tc:opendocument:xmlns:meta:1.0";
pub static CONFIG_NS: &'static str = "urn:oasis:names:tc:opendocument:xmlns:config:1.0";
pub static TEXT_NS: &'static str = "urn:oasis:names:tc:opendocument:xmlns:text:1.0";
pub static TABLE_NS: &'static str = "urn:oasis:names:tc:opendocument:xmlns:table:1.0";
pub static DRAWING_NS: &'static str = "urn:oasis:names:tc:opendocument:xmlns:drawing:1.0";
pub static PRESENTATION_NS: &'static str = "urn:oasis:names:tc:opendocument:xmlns:presentation:1.0";
pub static DR3D_NS: &'static str = "urn:oasis:names:tc:opendocument:xmlns:dr3d:1.0";
pub static ANIM_NS: &'static str = "urn:oasis:names:tc:opendocument:xmlns:animation:1.0";
pub static CHART_NS: &'static str = "urn:oasis:names:tc:opendocument:xmlns:chart:1.0";
pub static FORM_NS: &'static str = "urn:oasis:names:tc:opendocument:xmlns:form:1.0";
pub static SCRIPT_NS: &'static str = "urn:oasis:names:tc:opendocument:xmlns:script:1.0";
pub static STYLE_NS: &'static str = "urn:oasis:names:tc:opendocument:xmlns:style:1.0";
pub static NUMBER_NS: &'static str = "urn:oasis:names:tc:opendocument:xmlns:data style:1.0";
pub static MANIFEST_NS: &'static str = "urn:oasis:names:tc:opendocument:xmlns:manifest:1.0";
pub static FO_NS: &'static str = "urn:oasis:names:tc:opendocument:xmlns:xsl-fo-compatible:1.0";
pub static SVG_NS: &'static str = "urn:oasis:names:tc:opendocument:xmlns:svg-compatible:1.0";
pub static SMIL_NS: &'static str = "urn:oasis:names:tc:opendocument:xmlns:smil-compatible:1.0";
pub static DC_NS: &'static str = "http://purl.org/dc/elements/1.1/";
pub static XLINK_NS: &'static str = "http://www.w3.org/1999/xlink";
pub static MATH_NS: &'static str = "http://www.w3.org/1998/Math/MathML";
pub static XFORMS_NS: &'static str = "http://www.w3.org/2002/xforms";
/// Définit les préfixes sur le noeud racine du document XML
pub fn setup_prefixes(builder: minidom::ElementBuilder) -> minidom::ElementBuilder
{
    builder.prefix(Some("style".to_string()), STYLE_NS).unwrap()
    .prefix(Some("office".to_string()), OFFICE_NS).unwrap()
    .prefix(Some("meta".to_string()), META_NS).unwrap()
    .prefix(Some("config".to_string()), CONFIG_NS).unwrap()
    .prefix(Some("text".to_string()), TEXT_NS).unwrap()
    .prefix(Some("table".to_string()), TABLE_NS).unwrap()
    .prefix(Some("presentation".to_string()), PRESENTATION_NS).unwrap()
    .prefix(Some("dr3d".to_string()), DR3D_NS).unwrap()
    .prefix(Some("anim".to_string()), ANIM_NS).unwrap()
    .prefix(Some("chart".to_string()), CHART_NS).unwrap()
    .prefix(Some("form".to_string()), FORM_NS).unwrap()
    .prefix(Some("script".to_string()), SCRIPT_NS).unwrap()
    .prefix(Some("number".to_string()), NUMBER_NS).unwrap()
    .prefix(Some("manifest".to_string()), MANIFEST_NS).unwrap()
    .prefix(Some("fo".to_string()), FO_NS).unwrap()
    .prefix(Some("svg".to_string()), SVG_NS).unwrap()
    .prefix(Some("smil".to_string()), SMIL_NS).unwrap()
    .prefix(Some("dc".to_string()), DC_NS).unwrap()
    .prefix(Some("xlink".to_string()), XLINK_NS).unwrap()
    .prefix(Some("math".to_string()), MATH_NS).unwrap()
    .prefix(Some("xforms".to_string()), XFORMS_NS).unwrap()
}