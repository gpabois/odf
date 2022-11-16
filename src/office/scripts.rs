use odf_macros::{define_element, define_child_elements};
use super::event_listeners::EventListeners;
use std::str::FromStr;

#[define_element(
    namespace = "crate::ns::OFFICE_NS",
    name = "scripts",
    children = "ScriptsChildElement"
)]
#[derive(Default)]
pub struct Scripts {}

#[define_child_elements(EventListeners, Script)]
pub enum ScriptsChildElement {}

#[define_element(
    namespace = "crate::ns::OFFICE_NS",
    name = "script",
    attribute(name="language", prefix="script", r#type="String")
)]
#[derive(Default)]
pub struct Script {}
