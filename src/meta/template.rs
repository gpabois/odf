use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::META_NS",
    name = "template",
    attribute(name = "date", r#type="String", prefix = "meta"),
    attribute(name = "actuate", r#type="String", prefix = "xlink"),
    attribute(name = "href", r#type="String", prefix = "xlink"),
    attribute(name = "title", r#type="String", prefix = "xlink"),
    attribute(name = "type", r#type="String", prefix = "xlink")
)]
#[derive(Default)]
pub struct Template {}