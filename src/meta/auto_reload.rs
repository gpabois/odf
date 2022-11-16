use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::META_NS",
    name = "auto-reload",
    attribute(name = "delay", r#type="String", prefix="delay"),
    attribute(name = "actuate", r#type="String", prefix="xlink"),
    attribute(name = "href", r#type = "String", prefix="xlink"),
    attribute(name = "show", r#type="String", prefix="xlink"),
    attribute(name = "type", r#type="String", prefix="xlink")
)]
#[derive(Default)]
pub struct AutoReload {}