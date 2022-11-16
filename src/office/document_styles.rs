use super::{automatic_styles::AutomaticStyles, master_styles::MasterStyles, styles::Styles};

use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::OFFICE_NS",
    name="document-styles",
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