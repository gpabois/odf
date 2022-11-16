use odf_macros::{define_element, define_child_elements};


#[define_element(
    namespace = "crate::ns::CONFIG_NS",
    name = "config-item-set",
    attribute(name = "name", r#type="String", prefix="config"),
    children = "ConfigItemSetChildElement"
)]
#[derive(Default)]
pub struct ConfigItemSet {}

#[define_child_elements(
    ConfigItem,
    ConfigItemMapIndexed,
    ConfigItemMapNamed,
    ConfigItemSet
)]
pub enum ConfigItemSetChildElement {}

#[define_element(
    namespace = "crate::ns::CONFIG_NS",
    name = "config-item",
    content = "String",
    attribute(name = "name", r#type="String", prefix = "config"),
    attribute(name = "type", r#type="String", prefix = "config")
)]
#[derive(Default)]
pub struct ConfigItem {}

#[define_element(
    namespace = "crate::ns::CONFIG_NS",
    name = "config-item-map-indexed",
    attribute(name = "name", r#type="String", prefix = "config"),
    children = "ConfigItemMapEntry"
)]
#[derive(Default)]
pub struct ConfigItemMapIndexed {}

#[define_element(
    namespace = "crate::ns::CONFIG_NS",
    name = "config-item-map-named",
    attribute(name = "name", r#type="String", prefix = "config"),
    children = "ConfigItemMapEntry"
)]
#[derive(Default)]
pub struct ConfigItemMapNamed {}

#[define_element(
    namespace = "crate::ns::CONFIG_NS",
    name = "config-item-map-entry",
    attribute(name = "name", r#type="String", prefix = "config"),
    children = "ConfigItemMapEntryChildElement"
)]
#[derive(Default)]
pub struct ConfigItemMapEntry {}

#[define_child_elements(
    ConfigItem,
    ConfigItemMapIndexed,
    ConfigItemMapNamed,
    ConfigItemSet
)]
pub enum ConfigItemMapEntryChildElement {}