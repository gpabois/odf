use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::META_NS",
    name = "keyword"
)]
#[derive(Default)]
pub struct Keyword {}