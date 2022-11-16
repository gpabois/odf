use odf_macros::define_element;
use crate::ns::OFFICE_NS;
use super::settings::Settings;
use std::str::FromStr;

#[define_element(
    setup_prefixes="true",
    namespace = "OFFICE_NS",
    name="document-settings",
    attribute(name="transformation", r#type="String", prefix="grddl"),
    attribute(name="version", r#type="String", prefix="office"),
    child(r#type="Settings")
)]
#[derive(Default)]
pub struct DocumentSettings
{
    //settings: Settings
}