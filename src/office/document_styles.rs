use super::{automatic_styles::AutomaticStyles, master_styles::MasterStyles, styles::Styles};
use std::str::FromStr;

use odf_macros::define_element;

#[define_element(
    setup_prefixes="true",
    namespace = "crate::ns::OFFICE_NS",
    name="document-styles",
    attribute(name="transformation", r#type="String", prefix="grddl"),
    attribute(name="version", r#type="String", prefix="office"),
    child(r#type="AutomaticStyles"),
    child(r#type="MasterStyles"),
    child(r#type="Styles")
)]
#[derive(Default)]
pub struct DocumentStyles
{
    // automatic_styles: AutomaticStyles,
    // master_styles: MasterStyles,
    // styles: Styles
}