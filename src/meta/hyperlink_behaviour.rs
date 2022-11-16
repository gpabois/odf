use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::META_NS",
    name = "hyperlink-behaviour",
    attribute(name = "target-frame-name", prefix = "office", r#type="String"),
    attribute(name = "show", prefix = "xlinks", r#type="String")
)]
#[derive(Default)]
pub struct HyperlinkBehaviour {}