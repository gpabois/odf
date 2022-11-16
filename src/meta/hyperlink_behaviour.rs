use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::META_NS",
    name = "hyperlink-behaviour"
)]
#[derive(Default)]
pub struct HyperlinkBehaviour {}