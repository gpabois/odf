
use odf_macros::define_element;
use crate::config;

#[define_element(
    namespace = "crate::ns::OFFICE_NS",
    name="settings",
    children="config::ConfigItemSet"
)]
#[derive(Default)]
pub struct Settings {}