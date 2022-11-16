use odf_macros::define_element;
use crate::ns::OFFICE_NS;
use super::settings::Settings;

#[define_element(
    namespace = "OFFICE_NS",
    name="document-settings",
    child(r#type="Settings")
)]
#[derive(Default)]
pub struct DocumentSettings
{
    //settings: Settings
}