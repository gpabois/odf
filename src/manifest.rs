use odf_macros::define_element;

#[define_element(
    name = "manifest",
    namespace = "crate::ns::MANIFEST_NS"
)]
#[derive(Default)]
pub struct Manifest {}