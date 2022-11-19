use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::TEXT_NS",
    name = "meta-field"
)]
#[derive(Default)]
pub struct MetaField {}